use rand::Rng;
use gnuplot::{Figure, Caption, Color};
use linreg::{linear_regression, linear_regression_of};

enum Direction {
    Right,
    Left,
}

fn single_drunkard_step() -> Direction {
    if rand::thread_rng().gen_range(0.0, 1.0) > 0.5 {
        return Direction::Left;
    }
    else {
        return Direction::Right;
    }
}

fn road_of_drunkard(steps: u64) -> i64 {
    let mut position: i64 = 0;
    for _ in 0..steps {
        match single_drunkard_step() {
            Direction::Left => position -= 1,
            Direction::Right => position += 1,
        }
    }
    position
}

fn roads_of_drunkards(steps: u64, drunkards: &u64) -> Vec<i64> {
    let mut positions = Vec::new();
    for _ in 0..*drunkards {
        positions.push(road_of_drunkard(steps));
    }
    positions
}

fn full_simulation(steps_set: &Vec<u64>, drunkards: &u64) -> Vec<f64> {
    let mut std_deviations = Vec::new();
    for steps in steps_set {
        let position = roads_of_drunkards(*steps, drunkards);
        std_deviations.push( stats::stddev( position.into_iter() ) );
    }
    std_deviations
}

fn calculate_logs(steps_set: &Vec<u64>, std_deviations: &Vec<f64>) -> (Vec<f64>, Vec<f64>) {
        let mut steps_set_ln = Vec::new();
        let mut std_deviations_ln = Vec::new();
        for steps in steps_set {
            steps_set_ln.push( (*steps as f64).ln() );
        }
        for std_deviation in std_deviations {
            std_deviations_ln.push( std_deviation.ln());
        }
        (steps_set_ln, std_deviations_ln)
}

fn reglinp(x: &Vec<f64>, y: &Vec<f64>) -> (f64, f64) {
        linear_regression(&x,&y).unwrap()
}


fn plot(plot_data: (Vec<f64>, Vec<f64>)) {
    let mut fg = Figure::new();
    fg.axes2d()
        .lines_points(&plot_data.0, &plot_data.1, &[Color("black")]);
    fg.show();
}

fn theoretical_prediciton(steps_set: &Vec<u64>) -> Vec<f64> {
    let mut std_deviation_prediciton = Vec::new();
    for steps in steps_set {
        std_deviation_prediciton.push(((*steps as f64).sqrt()).ln())
    }
    std_deviation_prediciton
}

fn main()
{
    let steps_set = vec![10,100,1000,10_000,100_000];
    let drunkards = 100;
    let std_deviation = full_simulation(&steps_set, &drunkards);
    let result_data = calculate_logs(&steps_set, &std_deviation);
    println!("{:?}", reglinp(&result_data.0, &result_data.1));
    println!("{:?}", theoretical_prediciton(&steps_set));
    plot(result_data);
}
