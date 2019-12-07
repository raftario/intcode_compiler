use crate::error::Error;

pub fn parse(input: &str) -> Result<Vec<isize>, Error> {
    input
        .split(',')
        .enumerate()
        .map(|(i, s)| {
            s.replace("\n", "")
                .replace("\r", "")
                .parse()
                .map_err(|_| Error::InvalidInput {
                    token: s.to_owned(),
                    position: i,
                })
        })
        .try_fold(Vec::new(), |mut acc, x| match x {
            Ok(x) => {
                acc.push(x);
                Ok(acc)
            }
            Err(e) => Err(e),
        })
}

#[cfg(test)]
mod tests {
    use crate::{error::Error::InvalidInput, parser::parse};

    #[test]
    fn valid() {
        let input = "-2,-1,0,1,2";
        let expected = vec![-2, -1, 0, 1, 2];
        assert_eq!(expected, parse(input).unwrap());
    }

    #[test]
    fn invalid() {
        let input = "-2,-1,zero,1,2";
        let expected = InvalidInput {
            token: "zero".to_owned(),
            position: 2,
        };
        assert_eq!(expected, parse(input).unwrap_err());
    }
}
