#[derive(Clone, Debug, Default, PartialEq)]
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
    pub desc: Vec<f32>,
}

impl IPRecord {
    /// Return IPRecord as little endian bytes
    ///
    /// ```
    /// use aspmatch::IPRecord;
    /// let record = IPRecord::default();
    /// let expected = vec![0; 1 * 8 + 9 * 4 + 1]; // 1 u64, 9 u32/f32, 1 u8
    /// assert_eq!(record.as_le_bytes(), expected);
    /// ```
    pub fn as_le_bytes(&self) -> Vec<u8> {
        vec![
            self.x.to_le_bytes().to_vec(),
            self.y.to_le_bytes().to_vec(),
            self.xi.to_le_bytes().to_vec(),
            self.yi.to_le_bytes().to_vec(),
            self.orientation.to_le_bytes().to_vec(),
            self.scale.to_le_bytes().to_vec(),
            self.interest.to_le_bytes().to_vec(),
            self.polarity.to_le_bytes().to_vec(),
            self.octave.to_le_bytes().to_vec(),
            self.scale_lvl.to_le_bytes().to_vec(),
            (self.desc.len() as u64).to_le_bytes().to_vec(),
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

    pub fn as_text(&self) -> String {
        format!(
            "{} {} {} {} {} {} {} {} {} {} {} {}",
            self.x,
            self.y,
            self.xi,
            self.yi,
            self.orientation,
            self.scale,
            self.interest,
            self.polarity,
            self.octave,
            self.scale_lvl,
            self.desc.len(),
            self.desc
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct IPMatch {
    pub image_1: Vec<IPRecord>,
    pub image_2: Vec<IPRecord>,
}

impl IPMatch {
    /// Return IPMatch as little endian bytes
    ///
    /// ```
    /// use aspmatch::IPMatch;
    /// let _match = IPMatch::default();
    /// let expected = vec![0; 2 * 8]; // 2 * u64, empty records
    /// assert_eq!(_match.as_le_bytes(), expected);
    /// ```
    pub fn as_le_bytes(&self) -> Vec<u8> {
        let size_1_bytes = self.image_1.len().to_le_bytes().into_iter();
        let size_2_bytes = self.image_2.len().to_le_bytes().into_iter();
        let image_1_bytes = self.image_1.iter().map(|i| i.as_le_bytes()).flatten();
        let image_2_bytes = self.image_2.iter().map(|i| i.as_le_bytes()).flatten();
        size_1_bytes
            .chain(size_2_bytes.chain(image_1_bytes.chain(image_2_bytes)))
            .collect()
    }

    pub fn as_text(&self) -> String {
        let header = format!("{} {}", self.image_1.len(), self.image_2.len());
        let image_1_txt = self
            .image_1
            .iter()
            .map(|r| r.as_text())
            .collect::<Vec<String>>()
            .join("\n");
        let image_2_txt = self
            .image_2
            .iter()
            .map(|r| r.as_text())
            .collect::<Vec<String>>()
            .join("\n");
        vec![
            header,
            "\n".to_string(),
            image_1_txt,
            "\n".to_string(),
            image_2_txt,
        ]
        .concat()
    }
}
