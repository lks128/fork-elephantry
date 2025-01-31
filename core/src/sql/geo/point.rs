/**
 * Rust type for
 * [point](https://www.postgresql.org/docs/current/datatype-geometric.html#id-1.5.7.16.5).
 */
#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
#[derive(Clone, Debug, PartialEq)]
pub struct Point(geo_types::Point<f64>);

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self(geo_types::Point::new(x, y))
    }
}

impl std::ops::Deref for Point {
    type Target = geo_types::Point<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0.x(), self.0.y())
    }
}

impl From<geo_types::Coord<f64>> for Point {
    fn from(coordinate: geo_types::Coord<f64>) -> Self {
        Self(coordinate.into())
    }
}

#[cfg(feature = "arbitrary")]
impl<'a> arbitrary::Arbitrary<'a> for Point {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let point = Self::new(f64::arbitrary(u)?, f64::arbitrary(u)?);

        Ok(point)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
impl crate::ToSql for Point {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::POINT
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L1800
     */
    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L1826
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        let mut buf = Vec::new();

        crate::to_sql::write_f64(&mut buf, self.0.x())?;
        crate::to_sql::write_f64(&mut buf, self.0.y())?;

        Ok(Some(buf))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
impl crate::FromSql for Point {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L1790
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        let coordinates = crate::from_sql::not_null(raw)?
            .parse::<crate::Coordinates>()
            .map_err(|_| Self::error(ty, raw))?;

        if coordinates.len() != 1 {
            return Err(Self::error(ty, raw));
        }

        Ok(Self::new(coordinates[0].x, coordinates[0].y))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L1811
     */
    fn from_binary(_: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let mut buf = crate::from_sql::not_null(raw)?;

        let x = crate::from_sql::read_f64(&mut buf)?;
        let y = crate::from_sql::read_f64(&mut buf)?;

        Ok(Self::new(x, y))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
impl crate::entity::Simple for Point {}

#[cfg(test)]
mod test {
    crate::sql_test!(
        point,
        crate::Point,
        [
            ("'(0,0)'", crate::Point::new(0., 0.)),
            ("'(5.1, 10.12345)'", crate::Point::new(5.1, 10.12345)),
        ]
    );
}
