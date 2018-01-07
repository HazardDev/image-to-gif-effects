extern crate image;

pub mod zoom {

	use gif::{Encoder, Repeat, SetParameter};
	use gif;

	use image::{GenericImage, DynamicImage, FilterType};
	use std::fs::File;
	use std::borrow::Cow;
	
	pub fn process(source: DynamicImage, output: String) {

		println!("Began processing source v0.0.2.");
		let mut frame_array: Vec<gif::Frame> = Vec::new();
		println!("Made frame array");


		// let mut first_frame = gif::Frame::from_rgb(source.width() as u16, source.height() as u16, source.raw_pixels().into_iter().as_slice());
		
		// let mut first_frame = gif::Frame::default();

		// first_frame.height = source.height() as u16;
		// first_frame.width  = source.width() as u16;
		// let source_iter = source.raw_pixels().into_iter();
		// let temp_slice = source_iter.to_owned();
		// let owned_slice = temp_slice.as_slice();
		// first_frame.buffer = Cow::Borrowed(owned_slice);
		
		// println!("Made first frame");
		// frame_array.push(first_frame); // Push first frame

		println!("Pushed first frame, processing width and height");
		let width = source.width();
		let height = source.height();

		let frames = 180;
		let seconds = 3;

		let mut resized: DynamicImage;
		println!("Started Resize Operation");


		// for i in 0..frames {
		// 	println!("Processing frame: {:3}", i);
		// 	resized = source.resize(width + (4 * i), height + (4 * i), FilterType::Lanczos3);


		// 	let cropped: DynamicImage = resized.crop(0, 0, width, height);

		// 	let mut this_frame = gif::Frame::default();
		// 	this_frame.height = cropped.height() as u16;
		// 	this_frame.width = cropped.width() as u16;
		// 	let buffer = cropped.raw_pixels().clone();
		// 	this_frame.buffer = Cow::Borrowed(buffer.as_slice()); //expects an owned u8 collection [u8]

		// 	frame_array.push(this_frame);

		// }

		// let mut image = File::create(output).unwrap();
		// let mut gif_encoder = Encoder::new(&mut image, source.width() as u16, source.height() as u16, &[]).unwrap();
		// gif_encoder.set(Repeat::Infinite).unwrap();

		// println!("Started writing data.");
		// for frame in frame_array {
		// 	// // let gif_frame = gif::Frame::from_rgba(source.width() as u16, source.height() as u16, frame.into_buffer());
		// 	// let gif_frame = gif::Frame::default();
		// 	// gif_frame.buffer = Cow::Borrowed(frame.buffer);
		// 	// gif_frame.height = source.height() as u16;
		// 	// gif_frame.width  = source.width() as u16;
		// 	gif_encoder.write_frame(&frame).unwrap();
		// }


		let mut image = File::create(output).unwrap();
		let mut gif_encoder = Encoder::new(&mut image, source.width() as u16, source.height() as u16, &[]).unwrap();
		gif_encoder.set(Repeat::Infinite).unwrap();
		println!("Started writing data.");
		
		let mut buffer;
		for i in 0..frames {
			println!("Processing frame: {:3}", i);
			resized = source.resize(width + (4 * i), height + (4 * i), FilterType::Lanczos3);


			let cropped: DynamicImage = resized.crop(0, 0, width, height);

			// let mut this_frame = gif::Frame::default();
			// this_frame.height = cropped.height() as u16;
			// this_frame.width = cropped.width() as u16;
			// buffer = cropped.raw_pixels().clone();
			// this_frame.buffer = Cow::Borrowed(buffer.as_slice()); //expects an owned u8 collection [u8]

			// gif_encoder.write_frame(&this_frame).unwrap();

		}


	}

}

pub mod shake {
	extern crate image;
	use image::{GenericImage, DynamicImage, Frame, Frames, FilterType};	
	pub fn process(source: image::DynamicImage, output: String) {}
}