/// a list of tpc's, with legal ranges, not really an enum, so no way to cnvert into a class
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Tpc {
	TpcInvalid = -2,
	TpcFBB = -1,
	TpcCBB = 0,
	TpcGBB = 1,
	TpcDBB = 2,
	TpcABB = 3,
	TpcEBB = 4,
	TpcBBB = 5,
	TpcFB = 6,
	TpcCB = 7,
	TpcGB = 8,
	TpcDB = 9,
	TpcAB = 10,
	TpcEB = 11,
	TpcBB = 12,
	TpcF = 13,
	TpcC = 14,
	TpcG = 15,
	TpcD = 16,
	TpcA = 17,
	TpcE = 18,
	TpcB = 19,
	TpcFS = 20,
	TpcCS = 21,
	TpcGS = 22,
	TpcDS = 23,
	TpcAS = 24,
	TpcES = 25,
	TpcBS = 26,
	TpcFSS = 27,
	TpcCSS = 28,
	TpcGSS = 29,
	TpcDSS = 30,
	TpcASS = 31,
	TpcESS = 32,
	TpcBSS = 33,
}

pub const TPC_MIN: Tpc = Tpc::TpcFBB;
pub const TPC_MAX: Tpc = Tpc::TpcBSS;

/// the delta in tpc value to go 1 semitone up or down
pub const TPC_DELTA_SEMITONE: i32 = 7;
/// the delta in tpc value to reach the next (or prev) enharmonic spelling
pub const TPC_DELTA_ENHARMONIC: i32 = 12;
/// the delta in pitch value to go 1 octave up or down
pub const PITCH_DELTA_OCTAVE: i32 = 12;
/// the number of steps in an octave
pub const STEP_DELTA_OCTAVE: i32 = 7;