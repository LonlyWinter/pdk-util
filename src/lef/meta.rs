//! PDK Meta

use crate::{
    lef::macros::{
        lef_parse_arr, lef_parse_bool, lef_parse_char, lef_parse_f32, lef_parse_string,
        lef_parse_string_arr, lef_parse_u32,
    },
    lef_macro,
};

// class
lef_macro! {"value", pub LefClass,;
    Core = "CORE"
    CoreSpacer = "CORE SPACER"
    CoreAntennacell = "CORE ANTENNACELL"
    CoreWelltap = "CORE WELLTAP"
}

// direction
lef_macro! {"value", pub LefPinDirection,;
    Input = "INPUT"
    Output = "OUTPUT"
    InOut = "INOUT"
}

// use
lef_macro! {"value", pub LefUse,;
    Signal = "SIGNAL"
    Clock = "CLOCK"
    Power = "POWER"
    Ground = "GROUND"
}

// shape
lef_macro! {"value", pub LefShape,;
    Abutment = "ABUTMENT"
}

lef_macro! { pub LefPort,;
    ,,
    rect = [f32; 4] = lef_parse_arr = "RECT"
    ,
    layer = String = lef_parse_string = "LAYER"
    ,
}

lef_macro! { pub LefPin,;
    ,
    port = LefPort = "PORT"
    ,,
    direction = LefPinDirection = LefPinDirection::try_from = "DIRECTION"
    use_ty = LefUse = LefUse::try_from = "USE"
    shape = LefShape = LefShape::try_from = "SHAPE"
    an_ten_na_gate_area = f32 = lef_parse_f32 = "ANTENNAGATEAREA"
    an_ten_na_diff_area = f32 = lef_parse_f32 = "ANTENNADIFFAREA"
    ,
}
lef_macro! { pub LefMacro,;
    pins = LefPin = "PIN"
    ,
    obs = LefPort = "OBS"
    ,,
    cls = LefClass = LefClass::try_from = "CLASS"
    foreign = String = lef_parse_string = "FOREIGN"
    origin = [f32; 2] = lef_parse_arr = "ORIGIN"
    size = [f32; 2] = lef_parse_arr = "SIZE"
    symmetry = [String; 3] = lef_parse_string_arr = "SYMMETRY"
    site = String = lef_parse_string = "SITE"
    ,
}

lef_macro! { pub LibLef,;
    macros = LefMacro = "MACRO"
    ,,,
    version = String = lef_parse_string = "VERSION"
    now_wire_extension_at_pin =  bool = lef_parse_bool = "NOWIREEXTENSIONATPIN"
    names_case_sensitive =  bool = lef_parse_bool = "NAMESCASESENSITIVE"
    divider_char = char = lef_parse_char = "DIVIDERCHAR"
    database_microns = u32 = lef_parse_u32 = "DATABASEMICRONS"
    bus_bit_chars = String = lef_parse_string = "BUSBITCHARS"
    ,
}

// time
lef_macro! {"value_with", pub LefTime,;
    Nonoseconds = u32 = lef_parse_u32 = "NANOSECONDS"
    ,
}

// Capacitance
lef_macro! {"value_with", pub LefCapacitance,;
    PicoFarads = u32 = lef_parse_u32 = "PICOFARADS"
    Cpersqdist = f32 = lef_parse_f32 = "CPERSQDIST"
    ,
}

// Resistance
lef_macro! {"value_with", pub LefResistance,;
    Ohms = u32 = lef_parse_u32 = "OHMS"
    Prersq = f32 = lef_parse_f32 = "PRERSQ"
    Normal = f32 = lef_parse_f32 = ""
    ,
}

// DataBase
lef_macro! {"value_with", pub LefDataBase,;
    Microns = u32 = lef_parse_u32 = "MICRONS"
    ,
}

// UseMinSpacing
lef_macro! {"value_with", pub LefUseMinSpacing,;
    Obs = bool = lef_parse_bool = "OBS"
    ,
}

// Site Symmetry
lef_macro! {"value", pub LefSiteSymmetry,;
    Y = "Y"
}

lef_macro! { pub LefUnits,;
    ,,,
    time = LefTime = LefTime::try_from = "TIME"
    capacitance = LefCapacitance = LefCapacitance::try_from = "CAPACITANCE"
    resistance = LefResistance = LefResistance::try_from = "RESISTANCE"
    data_base = LefDataBase = LefDataBase::try_from = "DATABASE"
    ,
}

lef_macro! { pub LefProperty,;
    ,,,
    layer = String = lef_parse_string = "LAYER"
    ,
}

lef_macro! { pub LefSite,;
    ,,,
    symmetry = LefSiteSymmetry = LefSiteSymmetry::try_from = "SYMMETRY"
    cls = LefClass = LefClass::try_from = "CLASS"
    size = [f32; 2] = lef_parse_arr = "SIZE"
    ,
}

// Layer Type
lef_macro! {"value", pub LefLayerType,;
    MasterSlice = "MASTERSLICE"
    Cut = "CUT"
    Routing = "ROUTING"
}

// Layer Direction
lef_macro! {"value", pub LefLayerDirection,;
    Vertical = "VERTICAL"
    Horizontal = "HORIZONTAL"
}

// Layer Antenna Model
lef_macro! {"value", pub LefLayerAntennaModel,;
    Oxide1 = "OXIDE1"
}

// Layer Enclosure
lef_macro! {"value_with", pub LefLayerEnclosure,;
    Below = [f32; 2] = lef_parse_arr = "BELOW"
    Above = [f32; 2] = lef_parse_arr = "ABOVE"
    ,
}

// Layer Antenna Diff side area ratio
lef_macro! {"value_with", pub LefLayerAntennaDiffSideAreaRatio,;
    Pwl = [f32; 8] = lef_parse_arr = "PWL"
    ,
}

// Layer DC Current Density
lef_macro! {"value_with", pub LefLayerDCCurrentDensity,;
    Average = f32 = lef_parse_f32 = "AVERAGE"
    ,
}

// Layer AC Current Density
lef_macro! {"value_with", pub LefLayerACCurrentDensity,;
    Rms = f32 = lef_parse_f32 = "RMS"
    ,
}

lef_macro! { pub LefLayerSpacingTable,;
    ,,
    width = [f32; 2] = lef_parse_arr = "WIDTH"
    ,
    parallel_run_length = u32 = lef_parse_u32 = "PARALLELRUNLENGTH"

    ,
}

lef_macro! { pub LefLayer,;
    ,,,
    ty = LefLayerType = LefLayerType::try_from = "TYPE"
    direction = LefLayerDirection = LefLayerDirection::try_from = "DIRECTION"
    pitch = [f32; 2] = lef_parse_arr = "PITCH"
    offset = [f32; 2] = lef_parse_arr = "OFFSET"
    width = f32 = lef_parse_f32 = "WIDTH"
    area = f32 = lef_parse_f32 = "AREA"
    thickness = f32 = lef_parse_f32 = "THICKNESS"
    min_enclosed_area = f32 = lef_parse_f32 = "MINENCLOSUDAREA"
    edge_capacitance = f32 = lef_parse_f32 = "EDGECAPACITANCE"
    antenna_model = LefLayerAntennaModel = LefLayerAntennaModel::try_from = "ANTENANMODEL"
    property = [String; 2] = lef_parse_string_arr = "PROPERTY"
    capacitance = LefCapacitance = LefCapacitance::try_from = "CAPACITANCE"
    resistance = LefResistance = LefResistance::try_from = "RESISTANCE"
    antenna_diff_side_area_ratio = LefLayerAntennaDiffSideAreaRatio = LefLayerAntennaDiffSideAreaRatio::try_from = "ANTENNADIFFSIDEAREARATIO"
    dc_current_density = LefLayerDCCurrentDensity = LefLayerDCCurrentDensity::try_from = "DCCURRENTDENSITY"
    ac_current_density = LefLayerACCurrentDensity = LefLayerACCurrentDensity::try_from = "ACCURRENTDENSITY"
    density_check_window = [u32; 2] = lef_parse_arr = "DENSITYCHECKWINDOW"
    density_check_step = u32 = lef_parse_u32 = "DENSITYCHECKSTEP"
    ,
    spacing_table = LefLayerSpacingTable = "SPACINGTABLE"
}

lef_macro! { pub TechLef,;
    ,,,
    version = String = lef_parse_string = "VERSION"
    names_case_sensitive =  bool = lef_parse_bool = "NAMESCASESENSITIVE"
    divider_char = char = lef_parse_char = "DIVIDERCHAR"
    bus_bit_chars = String = lef_parse_string = "BUSBITCHARS"
    manufacturing_grid = f32 = lef_parse_f32 = "MANUFACTURINGGRID"
    use_min_spacing = LefUseMinSpacing = LefUseMinSpacing::try_from = "USEMINSPACING"
    ,
    units = LefUnits = "UNITS"
    property = LefProperty = "PROPERTYDEFINITIONS"
}
