use winnow::{
    ascii::multispace0,
    combinator::{cut_err, delimited, opt, separated, terminated, trace},
    error::ParserError,
    stream::{AsBStr, AsChar, Compare, Range, Stream, StreamIsPartial},
    Parser,
};

pub fn debug_list<'s, Input, Output, ParseNext, Error>(
    occurrences: impl Into<Range> + Clone,
    mut parser: ParseNext,
) -> impl Parser<Input, Vec<Output>, Error>
where
    Input: StreamIsPartial + Stream + Compare<char> + AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    ParseNext: Parser<Input, Output, Error>,
    Error: ParserError<Input>,
{
    trace("debug_list", move |input: &mut Input| {
        terminated(
            separated(
                occurrences.clone(),
                parser.by_ref(),
                (multispace0, ',', multispace0),
            ),
            opt((multispace0, ',', multispace0)),
        )
        .parse_next(input)
    })
}

pub fn debug_array<'s, Input, Output, ParseNext, Error>(
    occurrences: impl Into<Range> + Clone,
    mut parser: ParseNext,
) -> impl Parser<Input, Vec<Output>, Error>
where
    Input: StreamIsPartial + Stream + Compare<char> + AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    ParseNext: Parser<Input, Output, Error>,
    Error: ParserError<Input>,
{
    trace("debug_array", move |input: &mut Input| {
        delimited(
            ('[', multispace0),
            cut_err(debug_list(occurrences.clone(), parser.by_ref())),
            (multispace0, ']'),
        )
        .parse_next(input)
    })
}

// pub fn debug_tuple_struct<'s, Input, Output, ParseNext, Error>(
//     name: &str,
//     occurrences: impl Into<Range> + Clone,
//     mut parser: ParseNext,
// ) -> impl Parser<Input, Vec<Output>, Error>
// where
//     Input: StreamIsPartial + Stream + Compare<char> + AsBStr,
//     <Input as Stream>::Token: AsChar + Clone,
//     ParseNext: Parser<Input, Output, Error>,
//     Error: ParserError<Input>,
// {
//     trace("debug_list", move |input: &mut Input| {
//         delimited(
//             (name, '(', multispace0),
//             debug_list(
//                 occurrences
//             ),
//             (
//                 separated(
//                     occurrences.clone(),
//                     parser.by_ref(),
//                     (multispace0, ',', multispace0),
//                 ),
//                 opt((multispace0, ',', multispace0)),
//             ),
//             (multispace0, ']'),
//         )
//         .parse_next(input)
//     })
// }
