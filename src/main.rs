use plotters::prelude::*;

// Define a simple DataPoint struct
struct DataPoint {
    x: f64,
    y: f64,
}

// Function to calculate the linear regression coefficients
fn linear_regression(data: &[DataPoint]) -> (f64, f64) {
    let n = data.len() as f64;
    let x_sum: f64 = data.iter().map(|p| p.x).sum();
    let y_sum: f64 = data.iter().map(|p| p.y).sum();
    let xy_sum: f64 = data.iter().map(|p| p.x * p.y).sum();
    let x_squared_sum: f64 = data.iter().map(|p| p.x * p.x).sum();

    let slope = (n * xy_sum - x_sum * y_sum) / (n * x_squared_sum - x_sum * x_sum);
    let intercept = (y_sum - slope * x_sum) / n;

    (slope, intercept)
}

// Function to calculate the mean of a vector
fn mean(values: &[f64]) -> f64 {
    let sum: f64 = values.iter().sum();
    sum / (values.len() as f64)
}

// Function to calculate the covariance of two vectors
fn calc_covariance(x: &[f64], y: &[f64], x_mean: f64, y_mean: f64) -> f64 {
    let mut covariance = 0.0;
    for i in 0..x.len() {
        covariance += (x[i] - x_mean) * (y[i] - y_mean);
    }
    covariance / (x.len() as f64 - 1.0)
}

// Function to calculate the variance of a vector
fn calc_variance(values: &[f64], mean: f64) -> f64 {
    let mut variance = 0.0;
    for &value in values {
        variance += (value - mean).powi(2);
    }
    variance / (values.len() as f64 - 1.0)
}
// Function to plot the data and the linear regression line
// Function to plot the data and the linear regression line
fn plot_data_with_regression(
    data: &[DataPoint],
    slope: f64,
    intercept: f64,
    plot_title: &str,
    x_label: &str,
    y_label: &str,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a plotter area
    let output_file = &format!("./images/plots/{}",output_file);
    let root = BitMapBackend::new(output_file, (1600, 1200)).into_drawing_area();
    root.fill(&WHITE)?;

    // Define the plot area
    let mut chart = ChartBuilder::on(&root)
        .caption(plot_title, ("sans-serif", 20))
        .margin(5)
        .set_all_label_area_size(40)
        .build_cartesian_2d(0.0..6.0, 0.0..6.0)?;

    // Draw the scatter plot
    chart.configure_mesh().draw()?;
    chart.draw_series(PointSeries::of_element(
        data.iter().map(|p| (p.x, p.y)),
        5,
        &BLUE,
        &|c, s, st| {
            return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled());
        },
    ))?;

    // Draw the linear regression line
    chart.draw_series(LineSeries::new(
        vec![(0.0, intercept), (5.0, slope * 5.0 + intercept)],
        &RED,
    ))?;

    Ok(())
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example dataset: (x, y) pairs for the scatter plot
    let data = vec![
        DataPoint { x: 1.0, y: 2.0 },
        DataPoint { x: 2.0, y: 4.0 },
        DataPoint { x: 3.0, y: 5.0 },
        DataPoint { x: 4.0, y: 4.0 },
        DataPoint { x: 5.0, y: 5.0 },
    ];

    // Calculate the linear regression coefficients
    let (slope, intercept) = linear_regression(&data);

    // Print the results
    println!("Slope (Coefficient): {}", slope);
    println!("Intercept: {}", intercept);

    // Plot the data with the linear regression line
    plot_data_with_regression(
        &data,
        slope,
        intercept,
        "Scatter Plot with Linear Regression",
        "X Axis",
        "Y Axis",
        "scatter_plot.png",
    )?;

    Ok(())
}