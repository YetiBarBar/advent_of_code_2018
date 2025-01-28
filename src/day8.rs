use nom::{
    character::complete::{char, u64},
    combinator::map,
    multi::many_m_n,
    sequence::{separated_pair, terminated},
    IResult, Parser,
};

#[derive(Debug)]
struct Header {
    nodes: u64,
    metadata: u64,
}

#[derive(Debug)]
struct Node {
    _header: Header,
    nodes: Vec<Node>,
    metadata: Vec<u64>,
}

fn header(data: &str) -> IResult<&str, Header> {
    map(
        terminated(separated_pair(u64, char(' '), u64), char(' ')),
        |(a, b)| Header {
            nodes: a,
            metadata: b,
        },
    )
    .parse(data)
}

fn metadata(data: &str) -> IResult<&str, u64> {
    u64(data)
}

fn nodes(data: &str) -> IResult<&str, Node> {
    let (remain, header) = header(data)?;

    let (remain, nds) =
        many_m_n(header.nodes as usize, header.nodes as usize, nodes).parse(remain)?;

    let (remain, metadata) = many_m_n(
        header.metadata as usize,
        header.metadata as usize,
        terminated(metadata, char(' ')),
    )
    .parse(remain)?;

    IResult::Ok((
        remain,
        Node {
            _header: header,
            nodes: nds,
            metadata,
        },
    ))
}

impl Node {
    fn metadata_value(&self) -> u64 {
        self.metadata.iter().sum::<u64>() + self.nodes.iter().map(Node::metadata_value).sum::<u64>()
    }

    fn value(&self) -> u64 {
        if self.nodes.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata.iter().fold(0_u64, |acc, meta| {
                if meta == &0 {
                    acc
                } else {
                    acc + self.nodes.get(*meta as usize - 1).map_or(0, Node::value)
                }
            })
        }
    }
}

fn main() {
    // We replace newline with a space so its simpler to parse metadata
    let raw = include_str!("../data/day_2018_8.data").replace('\n', " ");

    // Our input is a single node
    let node = nodes(&raw).unwrap().1;
    // println!("{:?}", nodes.unwrap().1);

    // Sum the metadata
    println!("Part 1: {}", node.metadata_value());
    println!("Part 2: {}", node.value());
}
