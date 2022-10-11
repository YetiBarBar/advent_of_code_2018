use std::str::FromStr;

#[derive(Debug)]
struct Claim {
    id: usize,
    x: usize,
    y: usize,
    size_x: usize,
    size_y: usize,
}

impl FromStr for Claim {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rest) = s.split_once('@').unwrap();
        let (x, rest) = rest.split_once(',').unwrap();
        let (y, rest) = rest.split_once(':').unwrap();
        let (size_x, size_y) = rest.split_once('x').unwrap();
        let id = id.trim_start_matches('#').trim().parse().unwrap();
        let x = x.trim().parse().unwrap();
        let y = y.trim().parse().unwrap();
        let size_x = size_x.trim().parse().unwrap();
        let size_y = size_y.trim().parse().unwrap();
        Ok(Self {
            id,
            x,
            y,
            size_x,
            size_y,
        })
    }
}

struct Field<const N: usize> {
    inner: [[usize; N]; N],
}

impl<const N: usize> Field<N> {
    fn new() -> Self {
        Self { inner: [[0; N]; N] }
    }

    fn insert_claim(&mut self, claim: &Claim) {
        for x in claim.x..(claim.x + claim.size_x) {
            for y in claim.y..(claim.y + claim.size_y) {
                self.inner[x][y] += 1;
            }
        }
    }

    fn count_twice_claim(&self) -> usize {
        self.inner
            .iter()
            .flat_map(|row| row.iter().filter(|c| c >= &&2))
            .count()
    }

    fn is_claim_overlapping(&self, claim: &Claim) -> bool {
        for x in claim.x..(claim.x + claim.size_x) {
            for y in claim.y..(claim.y + claim.size_y) {
                if self.inner[x][y] != 1 {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    let data = include_str!("../data/day3.data");
    /*     let data = r#"#1 @ 1,3: 4x4
    #2 @ 3,1: 4x4
    #3 @ 5,5: 2x2"#; */
    let claims: Vec<Claim> = data.lines().map(str::parse).map(Result::unwrap).collect();

    // let mut field = Field::<10>::new();
    let mut field = Field::<1000>::new();
    for claim in &claims {
        field.insert_claim(claim);
    }

    println!("Part 1: {}", field.count_twice_claim());

    for claim in &claims {
        if field.is_claim_overlapping(claim) {
            println!("Part 2 candidate: {}", claim.id);
        }
    }
}
