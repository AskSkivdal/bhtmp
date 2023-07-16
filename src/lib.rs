use image::{ImageBuffer, Rgb, RgbImage};

pub struct DataRecord {
    pub raw: Vec<u8>,
}

pub struct Bhtmp {
    pub headers: Vec<u8>,
    pub records: Vec<DataRecord>,
}

impl Bhtmp {
    pub fn new(bytes: Vec<u8>) -> Self {
        // Get the header
        let headers = bytes.get(0..16).unwrap();
        // Get everything after the header.
        let data = bytes.get(16..).unwrap();
        
        // Read segments of 8 bytes into records.
        let mut records: Vec<DataRecord> = vec![];
        for i in data.chunks(8) {
            records.push(DataRecord {
                raw: i.iter().map(|v| *v).collect(),
            })
        }

        Self {
            headers: headers.iter().map(|v| *v).collect(),
            records: records,
        }
    }

    pub fn to_image(self: &Self, bitidx: usize) -> RgbImage {
        let mut image: RgbImage = ImageBuffer::new(100, 100);
        
        for (i, rec) in self.records.iter().enumerate() {
            image.put_pixel(
                (i % 100) as u32,
                (i / 100) as u32,
                Rgb([rec.raw[bitidx], rec.raw[bitidx], rec.raw[bitidx]]),
            )
        }

        return image;
    }
}
