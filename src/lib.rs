#[derive(PartialEq, Debug)]
pub struct USD(i32);
#[derive(PartialEq, Debug)]
pub struct GBP(i32);
#[derive(PartialEq, Debug)]
pub struct CAD(i32);


pub trait ToUSD {
    fn to_usd(&self) -> USD;
    fn convert<T:FromUSD>(&self) -> T {
        T::from_usd(&self.to_usd())
    }
}

pub trait FromUSD {
    fn from_usd(u: &USD) -> Self;
}


impl ToUSD for GBP {
    fn to_usd(&self) -> USD {
        USD((self.0 * 130) / 100)
    }
}

impl ToUSD for CAD {
    fn to_usd(&self) -> USD {
        USD((self.0 * 77) / 100)
    }
}


impl ToUSD for USD {
    fn to_usd(&self) -> USD {
        USD(self.0)
    }
}


impl FromUSD for CAD {
    fn from_usd(u: &USD) -> Self {
        CAD((u.0 * 130) / 100)
    }
}


impl FromUSD for GBP {
    fn from_usd(u: &USD) -> Self {
        GBP((u.0 * 77) / 100)
    }
}


impl FromUSD for USD {
    fn from_usd(u: &USD) -> Self {
        USD(u.0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let g = GBP(200);
        let u = g.to_usd();
        assert_eq!(u, USD(260));

        let c = CAD::from_usd(&u);
        assert_eq!(c, CAD(338));

        let c2:CAD = g.convert();
        assert_eq!(c2, c);

        let c3:GBP = c.convert();
        assert_eq!(c3, g);

        let c4:GBP = g.convert();
        assert_eq!(c4, g);

        let u2:USD = u.convert();
        assert_eq!(u2, u);
    }
}
