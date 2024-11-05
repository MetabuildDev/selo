use winnow::{
    ascii::multispace0,
    combinator::{separated, trace},
    error::ParserError,
    stream::{Accumulate, AsChar, Compare, FindSlice, Range, Stream, StreamIsPartial},
    token::{take_till, take_until},
    Parser,
};

// fn tuple_struct<'s, Input, Literal, Output, Accumulator, Error>(
//     mut name: &'s str,
//     nb_fields: usize,
// ) -> impl Parser<Input, Accumulator, Error> + '_
// where
//     Input: Stream<Slice = Output>
//         + StreamIsPartial
//         + Compare<&'s str>
//         + FindSlice<Literal>
//         + Compare<char>,
//     Input::Token: AsChar + Clone,
//     Literal: Clone,
//     Accumulator: Accumulate<Output>,
//     Error: ParserError<Input>,
// {
//     trace("tuple_struct", move |input: &mut Input| {
//         let _ = name.parse_next(input)?;
//         let _ = multispace0.parse_next(input)?;
//         separated(
//             nb_fields,
//             take_till(0.., [',', ' ', '\n', '\t']),
//             (multispace0, ','),
//         )
//         .parse_next(input)
//     })
// }

fn debug_list<'s, Input, Literal, Output, Accumulator, Error>(
    occurrences: impl Into<Range> + Clone,
) -> impl Parser<Input, Accumulator, Error>
where
    Input: Stream<Slice = Output>
        + StreamIsPartial
        + Compare<&'s str>
        + FindSlice<Literal>
        + Compare<char>,
    Input::Token: AsChar + Clone,
    Literal: Clone,
    Accumulator: Accumulate<Output>,
    Error: ParserError<Input>,
{
    trace("debug_list", move |input: &mut Input| {
        separated(
            occurrences.clone(),
            take_till(0.., [',', ' ', '\n', '\t']),
            (multispace0, ','),
        )
        .parse_next(input)
    })
}
