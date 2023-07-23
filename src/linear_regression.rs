use std::error::Error;
use plotters::backend::BitMapBackend;
use plotters::chart::ChartBuilder;
use plotters::style::full_palette::{RED, BLUE, WHITE};
use plotters::drawing::IntoDrawingArea;
use plotters::element::{Circle, EmptyElement};
use plotters::series::{LineSeries, PointSeries};


// Define a simple DataPoint struct
#[derive(Clone)]
pub(crate) struct DataPoint {
     x: f64,
     y: f64,
}
impl DataPoint{
    pub(crate) fn new(x:f64, y:f64) ->Self{
        DataPoint{x,y}
    }
}
pub(crate) struct LinearRegression{
    data: Box<Vec<DataPoint>>,
    coefficient: f64,
    intercept: f64,
    goodness_of_fit:f64,
    residuals:f64,
    p_values_and_significance:f64,
    confidence_intervals:f64,
    predictions:f64,
    multicollinearity:f64,
    outliers:f64,
    homoscedasticity:f64,
}
impl LinearRegression {
    pub(crate) fn new(data : &[DataPoint]) ->Self{
        let mut model = LinearRegression{
            data: Box::new(data.to_vec()),
            coefficient: 0.0,
            intercept: 0.0,
            goodness_of_fit: 0.0,
            residuals: 0.0,
            p_values_and_significance: 0.0,
            confidence_intervals: 0.0,
            predictions: 0.0,
            multicollinearity: 0.0,
            outliers: 0.0,
            homoscedasticity: 0.0,
        };
        model.calc_slope_intercept();
        model

    }
    pub(crate) fn get_slope_intercept(&self) -> (f64,f64){
        (self.coefficient,self.intercept)
    }

    // Function to calculate the linear regression coefficients
        fn calc_slope_intercept(&mut self){
        let n = self.data.len() as f64;
        let x_sum: f64 = self.data.iter().map(|p| p.x).sum();
        let y_sum: f64 = self.data.iter().map(|p| p.y).sum();
        let xy_sum: f64 = self.data.iter().map(|p| p.x * p.y).sum();
        let x_squared_sum: f64 =self.data.iter().map(|p| p.x * p.x).sum();

        self.coefficient = (n * xy_sum - x_sum * y_sum) / (n * x_squared_sum - x_sum * x_sum);
        self.intercept = (y_sum - self.coefficient * x_sum) / n;

    }

// Function to plot the data and the linear regression line

    pub(crate) fn plot_data_with_regression(
        self: &Self,
        data: &[DataPoint],
        slope: f64,
        intercept: f64,
        plot_title: &str,
        x_label: &str,
        y_label: &str,
        output_file: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create a plotter area
        let output_file = &format!("./images/plots/{}", output_file);
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
}


