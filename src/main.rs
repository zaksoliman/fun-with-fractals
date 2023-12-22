use num::complex::Complex;

fn mandlebrot_polynomial(z: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
    z * z + c
}

fn iterations_at_point(max_iter: usize, c: Complex<f64>) -> usize {
    let mut z = Complex::new(0.0, 0.0);

    for i in 0..=max_iter {
        z = mandlebrot_polynomial(z, c);

        if  z.norm() > 2.0 {
            return i;
        }
    }
    max_iter
}

// This method builds a matrix where each value is the number of iterations of the corresponding coordinates
// Step size is determined by n_cols and n_rows
fn generate_mandlebrot_set(
    max_iters: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    n_cols: usize,
    n_rows: usize,
) -> Vec<Vec<usize>> {
    // A row has n_cols columns
    let mut rows = Vec::with_capacity(n_rows);
    let x_range_size = x_max - x_min;
    let y_range_size = y_max - y_min;

    for pos_y in 0..n_rows {

        let mut row = Vec::with_capacity(n_cols);
        let y_ratio = pos_y as f64 / n_rows as f64;

        for pos_x in 0..n_cols {

            let x_ratio = pos_x as f64 / n_cols as f64;

            // Starting point plus ditance traveled so far on the axis
            let cx = x_min + x_range_size * x_ratio;
            let cy = y_min + y_range_size * y_ratio;

            let n_iters = iterations_at_point(max_iters, Complex::new(cx, cy));
            row.push(n_iters);
        }
        rows.push(row)
    }

    rows
}

fn render_mandlebrot(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => '•',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%',
            };
            line.push(val);
        }
        println!("{}", line);
    }
}

fn main() {
    let mandlebrot = generate_mandlebrot_set(1000, -2.0, 1.0, -1.0, 1.0, 100, 24);
    render_mandlebrot(mandlebrot);
}
