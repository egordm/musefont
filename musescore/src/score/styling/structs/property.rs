#[derive(Clone, Copy, Debug, Primitive, PartialEq, Eq, Hash)]
pub enum PropertyId {
	Subtype = 0,
	Selected = 1,
	Generated = 2,
	Color = 3,
	Visible = 4,
	Z = 5,
	Small = 6,
	ShowCourtesy = 7,
	KeysigMode = 8,
	LineType = 9,
	Pitch = 10,

	Tpc1 = 11,
	Tpc2 = 12,
	Line = 13,
	Fixed = 14,
	FixedLine = 15,
	HeadType = 16,
	HeadGroup = 17,
	VeloType = 18,
	VeloOffset = 19,
	ArticulationAnchor = 20,

	Direction = 21,
	StemDirection = 22,
	NoStem = 23,
	SlurDirection = 24,
	LeadingSpace = 25,
	Distribute = 26,
	MirrorHead = 27,
	DotPosition = 28,
	Tuning = 29,
	Pause = 30,

	BarlineType = 31,
	BarlineSpan = 32,
	BarlineSpanFrom = 33,
	BarlineSpanTo = 34,
	Offset = 35,
	Fret = 36,
	String = 37,
	Ghost = 38,
	Play = 39,
	TimesigNominal = 40,

	TimesigActual = 41,
	NumberType = 42,
	BracketType = 43,
	NormalNotes = 44,
	ActualNotes = 45,
	P1 = 46,
	P2 = 47,
	GrowLeft = 48,
	GrowRight = 49,
	BoxHeight = 50,

	BoxWidth = 51,
	TopGap = 52,
	BottomGap = 53,
	LeftMargin = 54,
	RightMargin = 55,
	TopMargin = 56,
	BottomMargin = 57,
	LayoutBreak = 58,
	Autoscale = 59,
	Size = 60,

	Scale = 61,
	LockAspectRatio = 62,
	SizeIsSpatium = 63,
	Text = 64,
	HtmlText = 65,
	UserModified = 66,
	BeamPos = 67,
	BeamMode = 68,
	BeamNoSlope = 69,
	/// used for stems
	UserLen = 70,

	/// used for spacer
	Space = 71,
	Tempo = 72,
	TempoFollowText = 73,
	AccidentalBracket = 74,
	AccidentalType = 75,
	NumeratorString = 76,
	DenominatorString = 77,
	/// used for FiguredBassItem
	Fbprefix = 78,
	Fbdigit = 79,
	Fbsuffix = 80,
	Fbcontinuationline = 81,

	Fbparenthesis1 = 82,
	Fbparenthesis2 = 83,
	Fbparenthesis3 = 84,
	Fbparenthesis4 = 85,
	Fbparenthesis5 = 86,
	OttavaType = 87,
	NumbersOnly = 88,
	TrillType = 89,
	VibratoType = 90,
	HairpinCircledtip = 91,

	HairpinType = 92,
	HairpinHeight = 93,
	HairpinContHeight = 94,
	VeloChange = 95,
	VeloChangeMethod = 96,
	VeloChangeSpeed = 97,
	DynamicType = 98,
	DynamicRange = 99,
	SingleNoteDynamics = 100,
	//100
	Placement = 101,
	Velocity = 102,
	JumpTo = 103,
	PlayUntil = 104,
	ContinueAt = 105,
	Label = 106,
	MarkerType = 107,
	ArpUserLen1 = 108,
	ArpUserLen2 = 109,
	RepeatEnd = 110,
	RepeatStart = 111,
	RepeatJump = 112,
	MeasureNumberMode = 113,
	GlissType = 114,
	GlissText = 115,

	GlissShowText = 116,
	Diagonal = 117,
	Groups = 118,
	LineStyle = 119,
	LineWidth = 120,
	LassoPos = 121,
	LassoSize = 122,
	TimeStretch = 123,
	OrnamentStyle = 124,

	Timesig = 125,
	TimesigGlobal = 126,
	TimesigStretch = 127,
	TimesigType = 128,
	SpannerTick = 129,
	SpannerTicks = 130,
	SpannerTrack2 = 131,
	Offset2 = 132,
	BreakMmr = 133,
	RepeatCount = 134,

	UserStretch = 135,
	NoOffset = 136,
	Irregular = 137,
	Anchor = 138,
	SlurUoff1 = 139,
	SlurUoff2 = 140,
	SlurUoff3 = 141,
	SlurUoff4 = 142,
	StaffMove = 143,
	Verse = 144,

	Syllabic = 145,
	LyricTicks = 146,
	VoltaEnding = 147,
	LineVisible = 148,
	Mag = 149,
	UseDrumset = 150,
	Duration = 151,
	DurationType = 152,
	Role = 153,
	Track = 154,

	GlissandoStyle = 155,
	FretStrings = 156,
	FretFrets = 157,
	FretNut = 158,
	FretOffset = 159,
	FretNumPos = 160,

	SystemBracket = 161,
	Gap = 162,
	Autoplace = 163,
	DashLineLen = 164,
	DashGapLen = 165,
	Tick = 166,
	PlaybackVoice1 = 167,
	PlaybackVoice2 = 168,
	PlaybackVoice3 = 169,

	PlaybackVoice4 = 170,
	Symbol = 171,
	PlayRepeats = 172,
	CreateSystemHeader = 173,
	StaffLines = 174,
	LineDistance = 175,
	StepOffset = 176,
	StaffShowBarlines = 177,
	StaffShowLedgerlines = 178,
	StaffStemless = 179,

	StaffNoteheadScheme = 180,
	StaffGenClef = 181,
	StaffGenTimesig = 182,
	StaffGenKeysig = 183,
	StaffYoffset = 184,
	StaffUserdist = 185,
	StaffBarlineSpan = 186,
	StaffBarlineSpanFrom = 187,
	StaffBarlineSpanTo = 188,
	BracketSpan = 189,

	BracketColumn = 190,
	InameLayoutPosition = 191,
	SubStyle = 192,

	FontFace = 193,
	FontSize = 194,
	FontStyle = 195,

	FrameType = 196,
	FrameWidth = 197,
	FramePadding = 198,
	FrameRound = 199,
	FrameFgColor = 200,
	//200
	FrameBgColor = 201,
	SizeSpatiumDependent = 202,
	Align = 203,
	SystemFlag = 204,
	BeginText = 205,

	BeginTextAlign = 206,
	BeginTextPlace = 207,
	BeginHookType = 208,
	BeginHookHeight = 209,
	BeginFontFace = 210,
	BeginFontSize = 211,
	BeginFontStyle = 212,
	BeginTextOffset = 213,

	ContinueText = 214,
	ContinueTextAlign = 215,
	ContinueTextPlace = 216,
	ContinueFontFace = 217,
	ContinueFontSize = 218,
	ContinueFontStyle = 219,
	ContinueTextOffset = 220,
	EndText = 221,

	EndTextAlign = 222,
	EndTextPlace = 223,
	EndHookType = 224,
	EndHookHeight = 225,
	EndFontFace = 226,
	EndFontSize = 227,
	EndFontStyle = 228,
	EndTextOffset = 229,

	PosAbove = 230,

	LocationStaves = 231,
	LocationVoices = 232,
	LocationMeasures = 233,
	LocationFractions = 234,
	LocationGrace = 235,
	LocationNote = 236,

	Voice = 237,
	Position = 238,

	ClefTypeConcert = 239,
	ClefTypeTransposing = 240,
	Key = 241,
	/// for Icon
	Action = 242,
	MinDistance = 243,

	ArpeggioType = 244,
	ChordLineType = 245,
	ChordLineStraight = 246,
	TremoloType = 247,
	TremoloPlacement = 248,
	HarmonyType = 249,

	StartWithLongNames = 250,
	StartWithMeasureOne = 251,

	End = 252,
}