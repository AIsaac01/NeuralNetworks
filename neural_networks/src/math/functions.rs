/********************* ACTIVATION FUNCTIONS *********************/

// fast sigmoid function
pub fn sigmoid(val: f32) -> f32 {
	// absolute value of 'val'
	let mut abs: f32 = val;
	if abs < 0.0 {
		abs *= -1.0;
	}
	val / (1.0 + abs)
}

// derivative of sigmoid function
pub fn d_sigmoid(val: f32) -> f32 {
	sigmoid(val) * (1.0 - sigmoid(val))
}

// Recitfied Linear Unit Function
pub fn relu(val: f32) -> f32 {
	if val < 0.0 {
		return 0.0;
	}
	val
}

// derivative of ReLU function
pub fn d_relu(val: f32) -> f32 {
	if val < 0.0 {
		return 0.0;
	}
	1.0
}

/********************* LOSS FUNCTIONS *********************/

pub fn mean_squared_error(val: f32, expected: f32) -> f32 {
	0.5 * (val - expected) * (val - expected)
}

pub fn d_mean_squared_error(val: f32, expected: f32) -> f32 {
	val - expected
}
