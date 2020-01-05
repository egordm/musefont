/// a list of tpc's, with legal ranges, not really an enum, so no way to cnvert into a class
#[derive(Clone, Copy, Debug, PartialEq, Primitive, Eq, Hash)]
pub enum Tpc {
	TpcFBB = 0,
	TpcCBB = 1,
	TpcGBB = 2,
	TpcDBB = 3,
	TpcABB = 4,
	TpcEBB = 5,
	TpcBBB = 6,
	TpcFB = 7,
	TpcCB = 8,
	TpcGB = 9,
	TpcDB = 10,
	TpcAB = 11,
	TpcEB = 12,
	TpcBB = 13,
	TpcF = 14,
	TpcC = 15,
	TpcG = 16,
	TpcD = 17,
	TpcA = 18,
	TpcE = 19,
	TpcB = 20,
	TpcFS = 21,
	TpcCS = 22,
	TpcGS = 23,
	TpcDS = 24,
	TpcAS = 25,
	TpcES = 26,
	TpcBS = 27,
	TpcFSS = 28,
	TpcCSS = 29,
	TpcGSS = 30,
	TpcDSS = 31,
	TpcASS = 32,
	TpcESS = 33,
	TpcBSS = 34,
	TpcInvalid = 35,
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