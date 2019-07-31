#[macro_use]
extern crate vst;
use vst::plugin::{Info, Plugin, Category};
use vst::buffer::AudioBuffer;

struct BitCruster{bit_reduction: f32, sampling_rate: f32}

impl Default for BitCruster{
    fn default() -> BitCruster {
        BitCruster {
            bit_reduction: 1.0,
            sampling_rate: 1.0
        }
    }
}

impl Plugin for BitCruster {
    fn get_info(&self) -> Info {
        Info {
            name: "BitCruster".to_string(),
            vendor: "adbc".to_string(),
            unique_id: 010010,
            inputs: 2, // stereo in
            outputs: 2, // stereo out
            parameters: 2, // num_parameters
            category: Category::Effect,
            ..Default::default()
        }
    }
    fn get_parameter(&self, index: i32) -> f32 {
    match index {
        0 => self.bit_reduction, 
        1 => self.sampling_rate,
        _ => 0.0,
        }
    }
    
    fn set_parameter(&mut self, index: i32, value: f32) {
    match index {
        0 => self.bit_reduction = value.max(0.01),
        1 => self.sampling_rate = value.max(0.01),
        _ => (),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
    match index {
        0 => "Bit Reduction".to_string(),
        1 => "Sampling Rate".to_string(),
        _ => "".to_string(),
        }
    }

    fn get_parameter_label(&self, index: i32) -> String {
    match index {
        0 => format!("{:.1} bits",(self.bit_reduction*14.0) + 2.0 ),
        1 => "%".to_string(),
        _ => "".to_string(),
        }
    }
    fn process(&mut self,  buffer: &mut AudioBuffer<f32>) {

    let (inputs, outputs) = buffer.split();
    let bits = (self.bit_reduction * 14.0) + 2.0 ; // set to 2 - 16  bits range
    let max_value = (bits * bits) - 1.0; // pow(bits, 2) is for losers
    
    for (input_buffer, output_buffer) in inputs.into_iter().zip(outputs.into_iter()) {
        for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {

            *output_sample = (((input_sample + 1.0) * (max_value/2.0)).floor() / max_value) - 1.0 ;
                
            }
        }
    }

}
plugin_main!(BitCruster); 
