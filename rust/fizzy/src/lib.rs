use std::ops::Rem;

type Predicate<'a, T> = Box<dyn Fn(T) -> bool + 'a>;

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<'a, T> {
    matcher: Predicate<'a, T>,
    subs: String,
}

impl<'a, T> Matcher<'a, T>
where
    T: ToString + Copy,
{
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<'a, T>
    where
        F: Fn(T) -> bool + 'a,
        S: ToString,
    {
        Self {
            matcher: Box::new(matcher),
            subs: subs.to_string(),
        }
    }

    pub fn apply(&self, value: T) -> String {
        if (self.matcher)(value) {
            self.subs.clone()
        } else {
            String::new()
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<'a, T> {
    matchers: Vec<Matcher<'a, T>>,
}

impl<'a, T: ToString> Fizzy<'a, T>
where
    T: ToString + Copy,
{
    pub fn new() -> Self {
        Fizzy {
            matchers: Vec::new(),
        }
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<'a, T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
    {
        iter.map(move |item| self.process(item))
    }

    fn process(&self, item: T) -> String {
        let result = self
            .matchers
            .iter()
            .map(|m| m.apply(item))
            .collect::<String>();

        if result.is_empty() {
            return item.to_string();
        }

        result
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<'a, T>() -> Fizzy<'a, T>
where
    T: Rem<Output = T> + PartialEq<T> + From<u8> + ToString + Copy,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n: T| n % T::from(3) == T::from(0), "fizz"))
        .add_matcher(Matcher::new(|n: T| n % T::from(5) == T::from(0), "buzz"))
}
