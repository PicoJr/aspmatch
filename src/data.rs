#[derive(Clone, Debug, PartialEq)]
pub struct IPRecord {
    pub x: f32,
    pub y: f32,
    pub xi: i32,
    pub yi: i32,
    pub orientation: f32,
    pub scale: f32,
    pub interest: f32,
    pub polarity: u8,
    pub octave: u32,
    pub scale_lvl: u32,
    pub ndesc: u64,
    pub desc: Vec<f32>,
}

impl IPRecord {
    pub fn as_bytes(&self) -> Vec<u8> {
        vec![
            self.x.to_le_bytes().to_vec(),           // x
            self.y.to_le_bytes().to_vec(),           // y
            self.xi.to_le_bytes().to_vec(),          // xi
            self.yi.to_le_bytes().to_vec(),          // yi
            self.orientation.to_le_bytes().to_vec(), // orientation
            self.scale.to_le_bytes().to_vec(),       // scale
            self.interest.to_le_bytes().to_vec(),    // interest
            self.polarity.to_le_bytes().to_vec(),    // polarity
            self.octave.to_le_bytes().to_vec(),      // octave
            self.scale_lvl.to_le_bytes().to_vec(),   // scale lvl
            self.ndesc.to_le_bytes().to_vec(),       // ndesc
            self.desc
                .iter()
                .map(|e| e.to_le_bytes().to_vec())
                .flatten()
                .collect(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IPMatch {
    pub image_1: Vec<IPRecord>,
    pub image_2: Vec<IPRecord>,
}

impl IPMatch {
    pub fn as_bytes(&self) -> Vec<u8> {
        let size_1_bytes = self.image_1.len().to_le_bytes().into_iter();
        let size_2_bytes = self.image_2.len().to_le_bytes().into_iter();
        let image_1_bytes = self.image_1.iter().map(|i| i.as_bytes()).flatten();
        let image_2_bytes = self.image_2.iter().map(|i| i.as_bytes()).flatten();
        size_1_bytes
            .chain(size_2_bytes.chain(image_1_bytes.chain(image_2_bytes)))
            .collect()
    }
}
