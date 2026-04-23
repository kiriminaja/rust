//! Volumetric calculator for multi-item packages.
//!
//! Estimate the smallest bounding box (`length` × `width` × `height`) that
//! fits all items by trying three stacking strategies (vertical, horizontal,
//! side-by-side) and returning the arrangement with the smallest volume.

/// A single item line in a multi-item package.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Item {
    /// Quantity. Values < 1 are treated as 1.
    pub qty: u32,
    pub length: f64,
    pub width: f64,
    pub height: f64,
}

/// Resulting bounding box.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Dimensions {
    pub length: f64,
    pub width: f64,
    pub height: f64,
}

/// Compute the smallest bounding box across vertical / horizontal / side
/// stacking. Empty input returns zero dimensions. Tie-break order is
/// vertical → horizontal → side.
pub fn calculate(items: &[Item]) -> Dimensions {
    if items.is_empty() {
        return Dimensions::default();
    }

    let (mut l_vert, mut w_vert, mut h_vert) = (0.0_f64, 0.0_f64, 0.0_f64);
    let (mut l_hor, mut w_hor, mut h_hor) = (0.0_f64, 0.0_f64, 0.0_f64);
    let (mut l_side, mut w_side, mut h_side) = (0.0_f64, 0.0_f64, 0.0_f64);

    for it in items {
        let qty = if it.qty < 1 { 1 } else { it.qty } as f64;
        let (l, w, h) = (it.length, it.width, it.height);

        h_vert += h * qty;
        l_vert = l_vert.max(l);
        w_vert = w_vert.max(w);

        l_hor += l * qty;
        h_hor = h_hor.max(h);
        w_hor = w_hor.max(w);

        w_side += w * qty;
        h_side = h_side.max(h);
        l_side = l_side.max(l);
    }

    let vol_vert = l_vert * w_vert * h_vert;
    let vol_hor = l_hor * w_hor * h_hor;
    let vol_side = l_side * w_side * h_side;

    if vol_vert <= vol_hor && vol_vert <= vol_side {
        Dimensions {
            length: l_vert,
            width: w_vert,
            height: h_vert,
        }
    } else if vol_hor <= vol_side {
        Dimensions {
            length: l_hor,
            width: w_hor,
            height: h_hor,
        }
    } else {
        Dimensions {
            length: l_side,
            width: w_side,
            height: h_side,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(calculate(&[]), Dimensions::default());
    }

    #[test]
    fn single_item() {
        let out = calculate(&[Item {
            qty: 1,
            length: 10.0,
            width: 5.0,
            height: 3.0,
        }]);
        assert_eq!(
            out,
            Dimensions {
                length: 10.0,
                width: 5.0,
                height: 3.0
            }
        );
    }

    #[test]
    fn vertical_wins() {
        let out = calculate(&[Item {
            qty: 2,
            length: 10.0,
            width: 10.0,
            height: 2.0,
        }]);
        assert_eq!(
            out,
            Dimensions {
                length: 10.0,
                width: 10.0,
                height: 4.0
            }
        );
    }

    #[test]
    fn horizontal_wins() {
        let out = calculate(&[
            Item {
                qty: 5,
                length: 2.0,
                width: 10.0,
                height: 10.0,
            },
            Item {
                qty: 1,
                length: 10.0,
                width: 1.0,
                height: 1.0,
            },
        ]);
        assert_eq!(
            out,
            Dimensions {
                length: 20.0,
                width: 10.0,
                height: 10.0
            }
        );
    }

    #[test]
    fn side_wins() {
        let out = calculate(&[
            Item {
                qty: 5,
                length: 10.0,
                width: 2.0,
                height: 10.0,
            },
            Item {
                qty: 1,
                length: 1.0,
                width: 10.0,
                height: 1.0,
            },
        ]);
        assert_eq!(
            out,
            Dimensions {
                length: 10.0,
                width: 20.0,
                height: 10.0
            }
        );
    }

    #[test]
    fn qty_zero_treated_as_one() {
        let out = calculate(&[Item {
            qty: 0,
            length: 10.0,
            width: 5.0,
            height: 3.0,
        }]);
        assert_eq!(
            out,
            Dimensions {
                length: 10.0,
                width: 5.0,
                height: 3.0
            }
        );
    }
}
