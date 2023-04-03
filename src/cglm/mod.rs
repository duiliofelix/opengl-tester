use gl::types::GLfloat;

pub struct Matrix {
    values: [[GLfloat; 4]; 4],
}

impl Matrix {
    pub fn identity() -> Matrix {
        Matrix {
            values: [
                [1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ],
        }
    }
}

struct GLVector3([GLfloat; 3]);

impl GLVector3 {
    fn normalize(&self) {
        let norm = GLVector3([0.; 3]);
    }
}

pub fn rotate(matrix: &Matrix, angle: f32, axis: GLVector3) {}

pub fn translate(translation: &Matrix, position: &[GLfloat; 3]) -> [GLfloat; 4] {
    [
        translation.values[0][0] * position[0],
        translation.values[1][1] * position[1],
        translation.values[2][2] * position[2],
        translation.values[3][3],
    ]
}
