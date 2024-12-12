use grid::Grid;

use crate::read_input;

mod grid;
mod vector;

pub fn day12() {
    let s = read_input("src/day12/input.txt");

    let mut g = Grid::new(s);

    let plots = g.collect_plots();

    let mut total_price = 0usize;

    for plot in plots {
        let perimeter = plot.calculate_perimeter();
        let area = plot.calculate_area();

        let price = area * perimeter;

        total_price += price;
    }

    dbg!(total_price);
}

#[cfg(test)]
mod test {
    use crate::read_input;

    use super::grid::Grid;

    #[test]
    fn test_case_one() {
        let s = read_input("src/day12/test_1.txt");

        let mut g = Grid::new(s);

        let plots = g.collect_plots();

        let mut total_price = 0usize;

        for plot in plots {
            let perimeter = plot.calculate_perimeter();
            let area = plot.calculate_area();

            let price = area * perimeter;

            total_price += price;

            match plot.c {
                'A' | 'C' => {
                    assert_eq!(perimeter, 10);
                    assert_eq!(area, 4);
                    assert_eq!(price, 40);
                }

                'B' => {
                    assert_eq!(perimeter, 8);
                    assert_eq!(area, 4);
                    assert_eq!(price, 32);
                }
                'E' => {
                    assert_eq!(perimeter, 8);
                    assert_eq!(area, 3);
                    assert_eq!(price, 24);
                }
                'D' => {
                    assert_eq!(perimeter, 4);
                    assert_eq!(area, 1);
                    assert_eq!(price, 4);
                }
                _ => {
                    panic!()
                }
            }
        }

        assert_eq!(total_price, 140);
    }

    #[test]
    fn test_case_two() {
        let s = read_input("src/day12/test_2.txt");

        let mut g = Grid::new(s);

        let plots = g.collect_plots();

        let mut total_price = 0usize;

        for plot in plots {
            let perimeter = plot.calculate_perimeter();
            let area = plot.calculate_area();

            let price = area * perimeter;

            total_price += price;

            match plot.c {
                'O' => {
                    assert_eq!(area, 21);
                    assert_eq!(perimeter, 36);
                    assert_eq!(price, 756);
                }

                'X' => {
                    assert_eq!(perimeter, 4);
                    assert_eq!(area, 1);
                    assert_eq!(price, 4);
                }

                _ => {
                    panic!()
                }
            }
        }

        assert_eq!(total_price, 772);
    }

    #[test]
    fn test_case_three() {
        let s = read_input("src/day12/test_3.txt");

        let mut g = Grid::new(s);

        let plots = g.collect_plots();

        let mut total_price = 0usize;

        for plot in plots {
            let perimeter = plot.calculate_perimeter();
            let area = plot.calculate_area();

            let price = area * perimeter;

            total_price += price;

            match plot.c {
                'R' => {
                    assert_eq!(area, 12);
                    assert_eq!(perimeter, 18);
                    assert_eq!(price, 216);
                }

                'S' => {
                    assert_eq!(area, 3);
                    assert_eq!(perimeter, 8);
                    assert_eq!(price, 24);
                }

                'F' => {
                    assert_eq!(area, 10);
                    assert_eq!(perimeter, 18);
                    assert_eq!(price, 180);
                }

                'V' => {
                    assert_eq!(area, 13);
                    assert_eq!(perimeter, 20);
                    assert_eq!(price, 260);
                }

                'J' => {
                    assert_eq!(area, 11);
                    assert_eq!(perimeter, 20);
                    assert_eq!(price, 220);
                }

                'E' => {
                    assert_eq!(area, 13);
                    assert_eq!(perimeter, 18);
                    assert_eq!(price, 234);
                }

                'M' => {
                    assert_eq!(area, 5);
                    assert_eq!(perimeter, 12);
                    assert_eq!(price, 60);
                }

                _ => {
                    continue;
                }
            }
        }

        assert_eq!(total_price, 1930);
    }
}
