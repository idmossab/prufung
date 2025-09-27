#[derive(Debug)]
pub struct ChessPosition {
    pub rank: i32,
    pub file: i32,
}

#[derive(Debug)]
pub struct Queen {
    pub position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if (0..8).contains(&rank) && (0..8).contains(&file){
            Some(Self{rank,file})
        }else{
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self{position}
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let pos_self=&self.position;
        let pos_other=&other.position;

        pos_self.rank==pos_other.rank ||
        pos_self.file==pos_other.file ||
        (pos_self.rank-pos_other.rank).abs()==(pos_self.file-pos_other.file).abs()
    }
}