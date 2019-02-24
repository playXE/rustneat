use rulinalg::matrix::{BaseMatrix, BaseMatrixMut, Matrix};

#[allow(missing_docs)]
#[derive(Debug)]
pub struct Ctrnn<'a> {
    pub y: &'a [f64],
    pub delta_t: f64,
    pub tau: &'a [f64], //time constant
    pub wij: &'a [f64], //weights
    pub theta: &'a [f64], //bias
    pub i: &'a [f64], //sensors
}


#[allow(missing_docs)]
impl<'a> Ctrnn<'a> {
    pub fn activate_nn(&self, steps: usize) -> Vec<f64> {
        let mut y = Ctrnn::vector_to_column_matrix(self.y);
        let theta = Ctrnn::vector_to_column_matrix(self.theta);
        let wij = Ctrnn::vector_to_matrix(self.wij);
        let i = Ctrnn::vector_to_column_matrix(self.i);
        let tau = Ctrnn::vector_to_column_matrix(self.tau);
        let delta_t_tau = tau.apply(&(|x| 1.0 / x)) * self.delta_t;
        for _ in 0..steps {
            let activations = (&y - &theta).apply(&Ctrnn::sigmoid);
            y = &y + delta_t_tau.elemul(
                &((&wij * activations) - &y + &i)
            );
        };
        y.into_vec()
    }

    fn sigmoid(y: f64) -> f64 {
        1.0 / (1.0 + (-y).exp())
    }

    fn vector_to_column_matrix(vector: &[f64]) -> Matrix<f64> {
        Matrix::new(vector.len(), 1, vector)
    }

    fn vector_to_matrix(vector: &[f64]) -> Matrix<f64> {
        let width = (vector.len() as f64).sqrt() as usize;
        Matrix::new(width, width, vector)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! assert_delta_vector {
        ($x:expr, $y:expr, $d:expr) => {
            for pos in 0..$x.len() {
                if !(($x[pos] - $y[pos]).abs() <= $d) {
                    panic!(
                        "Element at position {:?} -> {:?} \
                         is not equal to {:?}",
                        pos, $x[pos], $y[pos]
                    );
                }
            }
        };
    }

    #[test]
    fn neural_network_activation_should_return_correct_values() {
        let gamma = vec![0.0, 0.0, 0.0];
        let delta_t = 13.436;
        let tau = vec![61.694, 10.149, 16.851];
        let wij = vec![
                -2.94737, 2.70665, -0.57046,
                -3.27553, 3.67193, 1.83218,
                2.32476, 0.24739, 0.58587,
            ];
        let theta = vec![-0.695126, -0.677891, -0.072129];
        let i = vec![0.98856, 0.31540, 0.0];

        let ctrnn = Ctrnn {
            y: &gamma,
            delta_t: delta_t,
            tau: &tau,
            wij: &wij,
            theta: &theta,
            i: &i
        };


        assert_delta_vector!(
            ctrnn.activate_nn(1),
            vec![
                0.11369936163643651,
                2.005484819913534,
                1.6093879775504707
            ],
            0.00000000000000000001
        );

        assert_delta_vector!(
            ctrnn.activate_nn(2),
            vec![
                0.1934507441070605,
                1.3576310165979484,
                0.5777018738984351
            ],
            0.00000000000000000001
        );

        assert_delta_vector!(
            ctrnn.activate_nn(10),
            vec![
                0.1420953991261177,
                1.7396545651402162,
                1.003785142846598
            ],
            0.00000000000000000001
        );

        assert_delta_vector!(
            ctrnn.activate_nn(30),
            vec![
                0.1663596276449866,
                1.5334698009336039,
                1.0185193568793127
            ],
            0.00000000000000000001
        );

        // converges
        assert_delta_vector!(
            ctrnn.activate_nn(100),
            vec![
                0.16622293036274471,
                1.5347586991255193,
                1.0184153349709313
            ],
            0.00000000000000000001
        );
    }
}
