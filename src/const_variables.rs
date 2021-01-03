/*dxlib const variables*/

pub const TRUE: i32 = 1;
pub const FALSE: i32 = 0;
pub const MAX_IMAGE_NUM: i32 = 32768;
pub const MAX_2DSURFACE_NUM: i32 = 32768;
pub const MAX_3DSURFACE_NUM: i32 = 65536;
pub const MAX_IMAGE_DIVNUM: i32 = 64;
pub const MAX_SURFACE_NUM: i32 = 65536;
pub const MAX_SHADOWMAP_NUM: i32 = 8192;
pub const MAX_SOFTIMAGE_NUM: i32 = 8192;
pub const MAX_SOUND_NUM: i32 = 32768;
pub const MAX_SOFTSOUND_NUM: i32 = 8192;
pub const MAX_MUSIC_NUM: i32 = 256;
pub const MAX_MOVIE_NUM: i32 = 100;
pub const MAX_MASK_NUM: i32 = 512;
pub const MAX_FONT_NUM: i32 = 40;
pub const MAX_INPUT_NUM: i32 = 256;
pub const MAX_SOCKET_NUM: i32 = 8192;
pub const MAX_LIGHT_NUM: i32 = 4096;
pub const MAX_SHADER_NUM: i32 = 4096;
pub const MAX_CONSTANT_BUFFER_NUM: i32 = 8192;
pub const MAX_MODEL_BASE_NUM: i32 = 32768;
pub const MAX_MODEL_NUM: i32 = 32768;
pub const MAX_VERTEX_BUFFER_NUM: i32 = 16384;
pub const MAX_INDEX_BUFFER_NUM: i32 = 16384;
pub const MAX_FILE_NUM: i32 = 32768;
pub const MAX_JOYPAD_NUM: i32 = 16;
pub const DEFAULT_SCREEN_SIZE_X: i32 = 640;
pub const DEFAULT_SCREEN_SIZE_Y: i32 = 480;
pub const DEFAULT_COLOR_BITDEPTH: i32 = 16;
pub const DEFAULT_ZBUFFER_BITDEPTH: i32 = 16;
pub const DX_DEFAULT_FONT_HANDLE: i32 = 2;
pub const MAX_USERIMAGEREAD_FUNCNUM: i32 = 10;
pub const DX_WINDOWSVERSION_31: i32 = 0;
pub const DX_WINDOWSVERSION_95: i32 = 1;
pub const DX_WINDOWSVERSION_98: i32 = 2;
pub const DX_WINDOWSVERSION_ME: i32 = 3;
pub const DX_WINDOWSVERSION_NT31: i32 = 260;
pub const DX_WINDOWSVERSION_NT40: i32 = 261;
pub const DX_WINDOWSVERSION_2000: i32 = 262;
pub const DX_WINDOWSVERSION_XP: i32 = 263;
pub const DX_WINDOWSVERSION_VISTA: i32 = 264;
pub const DX_WINDOWSVERSION_7: i32 = 265;
pub const DX_WINDOWSVERSION_8: i32 = 266;
pub const DX_WINDOWSVERSION_8_1: i32 = 267;
pub const DX_WINDOWSVERSION_10: i32 = 268;
pub const DX_WINDOWSVERSION_NT_TYPE: i32 = 256;
pub const DX_DIRECTXVERSION_NON: i32 = 0;
pub const DX_DIRECTXVERSION_1: i32 = 65536;
pub const DX_DIRECTXVERSION_2: i32 = 131072;
pub const DX_DIRECTXVERSION_3: i32 = 196608;
pub const DX_DIRECTXVERSION_4: i32 = 262144;
pub const DX_DIRECTXVERSION_5: i32 = 327680;
pub const DX_DIRECTXVERSION_6: i32 = 393216;
pub const DX_DIRECTXVERSION_6_1: i32 = 393472;
pub const DX_DIRECTXVERSION_7: i32 = 458752;
pub const DX_DIRECTXVERSION_8: i32 = 524288;
pub const DX_DIRECTXVERSION_8_1: i32 = 524544;
pub const DX_DIRECT3D_NONE: i32 = 0;
pub const DX_DIRECT3D_9: i32 = 1;
pub const DX_DIRECT3D_9EX: i32 = 2;
pub const DX_DIRECT3D_11: i32 = 3;
pub const DX_DIRECT3D_11_FEATURE_LEVEL_9_1: i32 = 37120;
pub const DX_DIRECT3D_11_FEATURE_LEVEL_9_2: i32 = 37376;
pub const DX_DIRECT3D_11_FEATURE_LEVEL_9_3: i32 = 37632;
pub const DX_DIRECT3D_11_FEATURE_LEVEL_10_0: i32 = 40960;
pub const DX_DIRECT3D_11_FEATURE_LEVEL_10_1: i32 = 41216;
pub const DX_DIRECT3D_11_FEATURE_LEVEL_11_0: i32 = 45056;
pub const DX_DIRECT3D_11_FEATURE_LEVEL_11_1: i32 = 45312;
pub const DX_CHARSET_DEFAULT: i32 = 0;
pub const DX_CHARSET_SHFTJIS: i32 = 1;
pub const DX_CHARSET_HANGEUL: i32 = 2;
pub const DX_CHARSET_BIG5: i32 = 3;
pub const DX_CHARSET_GB2312: i32 = 4;
pub const DX_CHARSET_WINDOWS_1252: i32 = 5;
pub const DX_CHARSET_ISO_IEC_8859_15: i32 = 6;
pub const DX_CHARSET_NUM: i32 = 7;
pub const DX_CHARCODEFORMAT_SHIFTJIS: i32 = 932;
pub const DX_CHARCODEFORMAT_GB2312: i32 = 936;
pub const DX_CHARCODEFORMAT_UHC: i32 = 949;
pub const DX_CHARCODEFORMAT_BIG5: i32 = 950;
pub const DX_CHARCODEFORMAT_UTF16LE: i32 = 1200;
pub const DX_CHARCODEFORMAT_UTF16BE: i32 = 1201;
pub const DX_CHARCODEFORMAT_WINDOWS_1252: i32 = 1252;
pub const DX_CHARCODEFORMAT_ISO_IEC_8859_15: i32 = 32764;
pub const DX_CHARCODEFORMAT_UTF8: i32 = 65001;
pub const DX_CHARCODEFORMAT_ASCII: i32 = 32765;
pub const DX_CHARCODEFORMAT_UTF32LE: i32 = 32766;
pub const DX_CHARCODEFORMAT_UTF32BE: i32 = 32767;
pub const DX_MIDIMODE_MCI: i32 = 0;
pub const DX_MIDIMODE_DM: i32 = 1;
pub const DX_MIDIMODE_DIRECT_MUSIC_REVERB: i32 = 1;
pub const DX_MIDIMODE_DIRECT_MUSIC_NORMAL: i32 = 2;
pub const DX_MIDIMODE_NUM: i32 = 3;
pub const DX_DRAWMODE_NEAREST: i32 = 0;
pub const DX_DRAWMODE_BILINEAR: i32 = 1;
pub const DX_DRAWMODE_ANISOTROPIC: i32 = 2;
pub const DX_DRAWMODE_OTHER: i32 = 3;
pub const DX_DRAWMODE_NUM: i32 = 4;
pub const DX_FONTTYPE_NORMAL: i32 = 0;
pub const DX_FONTTYPE_EDGE: i32 = 1;
pub const DX_FONTTYPE_ANTIALIASING: i32 = 2;
pub const DX_FONTTYPE_ANTIALIASING_4X4: i32 = 18;
pub const DX_FONTTYPE_ANTIALIASING_8X8: i32 = 34;
pub const DX_FONTTYPE_ANTIALIASING_EDGE: i32 = 3;
pub const DX_FONTTYPE_ANTIALIASING_EDGE_4X4: i32 = 19;
pub const DX_FONTTYPE_ANTIALIASING_EDGE_8X8: i32 = 35;
pub const DX_FONTIMAGE_BIT_1: i32 = 0;
pub const DX_FONTIMAGE_BIT_4: i32 = 1;
pub const DX_FONTIMAGE_BIT_8: i32 = 2;
pub const DX_BLENDMODE_NOBLEND: i32 = 0;
pub const DX_BLENDMODE_ALPHA: i32 = 1;
pub const DX_BLENDMODE_ADD: i32 = 2;
pub const DX_BLENDMODE_SUB: i32 = 3;
pub const DX_BLENDMODE_MUL: i32 = 4;
pub const DX_BLENDMODE_SUB2: i32 = 5;
pub const DX_BLENDMODE_XOR: i32 = 6;
pub const DX_BLENDMODE_DESTCOLOR: i32 = 8;
pub const DX_BLENDMODE_INVDESTCOLOR: i32 = 9;
pub const DX_BLENDMODE_INVSRC: i32 = 10;
pub const DX_BLENDMODE_MULA: i32 = 11;
pub const DX_BLENDMODE_ALPHA_X4: i32 = 12;
pub const DX_BLENDMODE_ADD_X4: i32 = 13;
pub const DX_BLENDMODE_SRCCOLOR: i32 = 14;
pub const DX_BLENDMODE_HALF_ADD: i32 = 15;
pub const DX_BLENDMODE_SUB1: i32 = 16;
pub const DX_BLENDMODE_PMA_ALPHA: i32 = 17;
pub const DX_BLENDMODE_PMA_ADD: i32 = 18;
pub const DX_BLENDMODE_PMA_SUB: i32 = 19;
pub const DX_BLENDMODE_PMA_INVSRC: i32 = 20;
pub const DX_BLENDMODE_PMA_ALPHA_X4: i32 = 21;
pub const DX_BLENDMODE_PMA_ADD_X4: i32 = 22;
pub const DX_BLENDMODE_NUM: i32 = 23;
pub const DX_DRAWFLOATCOORDTYPE_DIRECT3D9: i32 = 0;
pub const DX_DRAWFLOATCOORDTYPE_DIRECT3D10: i32 = 1;
pub const DX_BLENDGRAPHTYPE_NORMAL: i32 = 0;
pub const DX_BLENDGRAPHTYPE_WIPE: i32 = 1;
pub const DX_BLENDGRAPHTYPE_ALPHA: i32 = 2;
pub const DX_BLENDGRAPHTYPE_NUM: i32 = 3;
pub const DX_BLENDGRAPH_POSMODE_DRAWGRAPH: i32 = 0;
pub const DX_BLENDGRAPH_POSMODE_SCREEN: i32 = 1;
pub const DX_BLENDGRAPH_POSMODE_NUM: i32 = 2;
pub const DX_GRAPH_FILTER_MONO: i32 = 0;
pub const DX_GRAPH_FILTER_GAUSS: i32 = 1;
pub const DX_GRAPH_FILTER_DOWN_SCALE: i32 = 2;
pub const DX_GRAPH_FILTER_BRIGHT_CLIP: i32 = 3;
pub const DX_GRAPH_FILTER_BRIGHT_SCALE: i32 = 4;
pub const DX_GRAPH_FILTER_HSB: i32 = 5;
pub const DX_GRAPH_FILTER_INVERT: i32 = 6;
pub const DX_GRAPH_FILTER_LEVEL: i32 = 7;
pub const DX_GRAPH_FILTER_TWO_COLOR: i32 = 8;
pub const DX_GRAPH_FILTER_GRADIENT_MAP: i32 = 9;
pub const DX_GRAPH_FILTER_PREMUL_ALPHA: i32 = 10;
pub const DX_GRAPH_FILTER_INTERP_ALPHA: i32 = 11;
pub const DX_GRAPH_FILTER_YUV_TO_RGB: i32 = 12;
pub const DX_GRAPH_FILTER_Y2UV1_TO_RGB: i32 = 13;
pub const DX_GRAPH_FILTER_YUV_TO_RGB_RRA: i32 = 14;
pub const DX_GRAPH_FILTER_Y2UV1_TO_RGB_RRA: i32 = 15;
pub const DX_GRAPH_FILTER_BICUBIC_SCALE: i32 = 16;
pub const DX_GRAPH_FILTER_LANCZOS3_SCALE: i32 = 17;
pub const DX_GRAPH_FILTER_NUM: i32 = 18;
pub const DX_GRAPH_BLEND_NORMAL: i32 = 0;
pub const DX_GRAPH_BLEND_RGBA_SELECT_MIX: i32 = 1;
pub const DX_GRAPH_BLEND_MULTIPLE: i32 = 2;
pub const DX_GRAPH_BLEND_DIFFERENCE: i32 = 3;
pub const DX_GRAPH_BLEND_ADD: i32 = 4;
pub const DX_GRAPH_BLEND_SCREEN: i32 = 5;
pub const DX_GRAPH_BLEND_OVERLAY: i32 = 6;
pub const DX_GRAPH_BLEND_DODGE: i32 = 7;
pub const DX_GRAPH_BLEND_BURN: i32 = 8;
pub const DX_GRAPH_BLEND_DARKEN: i32 = 9;
pub const DX_GRAPH_BLEND_LIGHTEN: i32 = 10;
pub const DX_GRAPH_BLEND_SOFTLIGHT: i32 = 11;
pub const DX_GRAPH_BLEND_HARDLIGHT: i32 = 12;
pub const DX_GRAPH_BLEND_EXCLUSION: i32 = 13;
pub const DX_GRAPH_BLEND_NORMAL_ALPHACH: i32 = 14;
pub const DX_GRAPH_BLEND_ADD_ALPHACH: i32 = 15;
pub const DX_GRAPH_BLEND_MULTIPLE_A_ONLY: i32 = 16;
pub const DX_GRAPH_BLEND_PMA_MULTIPLE_A_ONLY: i32 = 17;
pub const DX_GRAPH_BLEND_NUM: i32 = 18;
pub const DX_RGBA_SELECT_SRC_R: i32 = 0;
pub const DX_RGBA_SELECT_SRC_G: i32 = 1;
pub const DX_RGBA_SELECT_SRC_B: i32 = 2;
pub const DX_RGBA_SELECT_SRC_A: i32 = 3;
pub const DX_RGBA_SELECT_BLEND_R: i32 = 4;
pub const DX_RGBA_SELECT_BLEND_G: i32 = 5;
pub const DX_RGBA_SELECT_BLEND_B: i32 = 6;
pub const DX_RGBA_SELECT_BLEND_A: i32 = 7;
pub const DX_FILL_WIREFRAME: i32 = 2;
pub const DX_FILL_SOLID: i32 = 3;
pub const DX_CULLING_NONE: i32 = 0;
pub const DX_CULLING_LEFT: i32 = 1;
pub const DX_CULLING_RIGHT: i32 = 2;
pub const DX_CULLING_NUM: i32 = 3;
pub const DX_CAMERACLIP_LEFT: i32 = 1;
pub const DX_CAMERACLIP_RIGHT: i32 = 2;
pub const DX_CAMERACLIP_BOTTOM: i32 = 4;
pub const DX_CAMERACLIP_TOP: i32 = 8;
pub const DX_CAMERACLIP_BACK: i32 = 16;
pub const DX_CAMERACLIP_FRONT: i32 = 32;
pub const DX_MV1_VERTEX_TYPE_1FRAME: i32 = 0;
pub const DX_MV1_VERTEX_TYPE_4FRAME: i32 = 1;
pub const DX_MV1_VERTEX_TYPE_8FRAME: i32 = 2;
pub const DX_MV1_VERTEX_TYPE_FREE_FRAME: i32 = 3;
pub const DX_MV1_VERTEX_TYPE_NMAP_1FRAME: i32 = 4;
pub const DX_MV1_VERTEX_TYPE_NMAP_4FRAME: i32 = 5;
pub const DX_MV1_VERTEX_TYPE_NMAP_8FRAME: i32 = 6;
pub const DX_MV1_VERTEX_TYPE_NMAP_FREE_FRAME: i32 = 7;
pub const DX_MV1_VERTEX_TYPE_NUM: i32 = 8;
pub const DX_MV1_MESHCATEGORY_NORMAL: i32 = 0;
pub const DX_MV1_MESHCATEGORY_OUTLINE: i32 = 1;
pub const DX_MV1_MESHCATEGORY_OUTLINE_ORIG_SHADER: i32 = 2;
pub const DX_MV1_MESHCATEGORY_NUM: i32 = 3;
pub const MV1_SAVETYPE_MESH: i32 = 1;
pub const MV1_SAVETYPE_ANIM: i32 = 2;
pub const MV1_SAVETYPE_NORMAL: i32 = (MV1_SAVETYPE_MESH | MV1_SAVETYPE_ANIM);
pub const MV1_ANIMKEY_DATATYPE_ROTATE: i32 = 0;
pub const MV1_ANIMKEY_DATATYPE_ROTATE_X: i32 = 1;
pub const MV1_ANIMKEY_DATATYPE_ROTATE_Y: i32 = 2;
pub const MV1_ANIMKEY_DATATYPE_ROTATE_Z: i32 = 3;
pub const MV1_ANIMKEY_DATATYPE_SCALE: i32 = 5;
pub const MV1_ANIMKEY_DATATYPE_SCALE_X: i32 = 6;
pub const MV1_ANIMKEY_DATATYPE_SCALE_Y: i32 = 7;
pub const MV1_ANIMKEY_DATATYPE_SCALE_Z: i32 = 8;
pub const MV1_ANIMKEY_DATATYPE_TRANSLATE: i32 = 10;
pub const MV1_ANIMKEY_DATATYPE_TRANSLATE_X: i32 = 11;
pub const MV1_ANIMKEY_DATATYPE_TRANSLATE_Y: i32 = 12;
pub const MV1_ANIMKEY_DATATYPE_TRANSLATE_Z: i32 = 13;
pub const MV1_ANIMKEY_DATATYPE_MATRIX4X4C: i32 = 15;
pub const MV1_ANIMKEY_DATATYPE_MATRIX3X3: i32 = 17;
pub const MV1_ANIMKEY_DATATYPE_SHAPE: i32 = 18;
pub const MV1_ANIMKEY_DATATYPE_OTHRE: i32 = 20;
pub const MV1_ANIMKEY_TIME_TYPE_ONE: i32 = 0;
pub const MV1_ANIMKEY_TIME_TYPE_KEY: i32 = 1;
pub const MV1_ANIMKEY_TYPE_QUATERNION_X: i32 = 0;
pub const MV1_ANIMKEY_TYPE_VECTOR: i32 = 1;
pub const MV1_ANIMKEY_TYPE_MATRIX4X4C: i32 = 2;
pub const MV1_ANIMKEY_TYPE_MATRIX3X3: i32 = 3;
pub const MV1_ANIMKEY_TYPE_FLAT: i32 = 4;
pub const MV1_ANIMKEY_TYPE_LINEAR: i32 = 5;
pub const MV1_ANIMKEY_TYPE_BLEND: i32 = 6;
pub const MV1_ANIMKEY_TYPE_QUATERNION_VMD: i32 = 7;
pub const DX_SCREEN_FRONT: i32 = -4;
pub const DX_SCREEN_BACK: i32 = -2;
pub const DX_SCREEN_WORK: i32 = -3;
pub const DX_SCREEN_TEMPFRONT: i32 = -16;
pub const DX_SCREEN_OTHER: i32 = -6;
pub const DX_NONE_GRAPH: i32 = -5;
pub const DX_SHAVEDMODE_NONE: i32 = 0;
pub const DX_SHAVEDMODE_DITHER: i32 = 1;
pub const DX_SHAVEDMODE_DIFFUS: i32 = 2;
pub const DX_IMAGESAVETYPE_BMP: i32 = 0;
pub const DX_IMAGESAVETYPE_JPEG: i32 = 1;
pub const DX_IMAGESAVETYPE_PNG: i32 = 2;
pub const DX_IMAGESAVETYPE_DDS: i32 = 3;
pub const DX_PLAYTYPE_LOOPBIT: i32 = 2;
pub const DX_PLAYTYPE_BACKBIT: i32 = 1;
pub const DX_PLAYTYPE_NORMAL: i32 = 0;
pub const DX_PLAYTYPE_BACK: i32 = (DX_PLAYTYPE_BACKBIT);
pub const DX_PLAYTYPE_LOOP: i32 = (DX_PLAYTYPE_LOOPBIT | DX_PLAYTYPE_BACKBIT);
pub const DX_MOVIEPLAYTYPE_BCANCEL: i32 = 0;
pub const DX_MOVIEPLAYTYPE_NORMAL: i32 = 1;
pub const DX_SOUNDTYPE_NORMAL: i32 = 0;
pub const DX_SOUNDTYPE_STREAMSTYLE: i32 = 1;
pub const DX_SOUNDDATATYPE_MEMNOPRESS: i32 = 0;
pub const DX_SOUNDDATATYPE_MEMNOPRESS_PLUS: i32 = 1;
pub const DX_SOUNDDATATYPE_MEMPRESS: i32 = 2;
pub const DX_SOUNDDATATYPE_FILE: i32 = 3;
pub const DX_READSOUNDFUNCTION_PCM: i32 = 1;
pub const DX_READSOUNDFUNCTION_OGG: i32 = 1;
pub const DX_READSOUNDFUNCTION_OPUS: i32 = 1;
pub const DX_READSOUNDFUNCTION_DEFAULT_NUM: i32 = 3;
pub const DX_REVERB_PRESET_DEFAULT: i32 = 0;
pub const DX_REVERB_PRESET_GENERIC: i32 = 1;
pub const DX_REVERB_PRESET_PADDEDCELL: i32 = 2;
pub const DX_REVERB_PRESET_ROOM: i32 = 3;
pub const DX_REVERB_PRESET_BATHROOM: i32 = 4;
pub const DX_REVERB_PRESET_LIVINGROOM: i32 = 5;
pub const DX_REVERB_PRESET_STONEROOM: i32 = 6;
pub const DX_REVERB_PRESET_AUDITORIUM: i32 = 7;
pub const DX_REVERB_PRESET_CONCERTHALL: i32 = 8;
pub const DX_REVERB_PRESET_CAVE: i32 = 9;
pub const DX_REVERB_PRESET_ARENA: i32 = 10;
pub const DX_REVERB_PRESET_HANGAR: i32 = 11;
pub const DX_REVERB_PRESET_CARPETEDHALLWAY: i32 = 12;
pub const DX_REVERB_PRESET_HALLWAY: i32 = 13;
pub const DX_REVERB_PRESET_STONECORRIDOR: i32 = 14;
pub const DX_REVERB_PRESET_ALLEY: i32 = 15;
pub const DX_REVERB_PRESET_FOREST: i32 = 16;
pub const DX_REVERB_PRESET_CITY: i32 = 17;
pub const DX_REVERB_PRESET_MOUNTAINS: i32 = 18;
pub const DX_REVERB_PRESET_QUARRY: i32 = 19;
pub const DX_REVERB_PRESET_PLAIN: i32 = 20;
pub const DX_REVERB_PRESET_PARKINGLOT: i32 = 21;
pub const DX_REVERB_PRESET_SEWERPIPE: i32 = 22;
pub const DX_REVERB_PRESET_UNDERWATER: i32 = 23;
pub const DX_REVERB_PRESET_SMALLROOM: i32 = 24;
pub const DX_REVERB_PRESET_MEDIUMROOM: i32 = 25;
pub const DX_REVERB_PRESET_LARGEROOM: i32 = 26;
pub const DX_REVERB_PRESET_MEDIUMHALL: i32 = 27;
pub const DX_REVERB_PRESET_LARGEHALL: i32 = 28;
pub const DX_REVERB_PRESET_PLATE: i32 = 29;
pub const DX_REVERB_PRESET_NUM: i32 = 30;
pub const DX_MASKTRANS_WHITE: i32 = 0;
pub const DX_MASKTRANS_BLACK: i32 = 1;
pub const DX_MASKTRANS_NONE: i32 = 2;
pub const DX_MASKGRAPH_CH_A: i32 = 0;
pub const DX_MASKGRAPH_CH_R: i32 = 1;
pub const DX_MASKGRAPH_CH_G: i32 = 2;
pub const DX_MASKGRAPH_CH_B: i32 = 3;
pub const DX_ZWRITE_MASK: i32 = 0;
pub const DX_ZWRITE_CLEAR: i32 = 1;
pub const DX_CMP_NEVER: i32 = 1;
pub const DX_CMP_LESS: i32 = 2;
pub const DX_CMP_EQUAL: i32 = 3;
pub const DX_CMP_LESSEQUAL: i32 = 4;
pub const DX_CMP_GREATER: i32 = 5;
pub const DX_CMP_NOTEQUAL: i32 = 6;
pub const DX_CMP_GREATEREQUAL: i32 = 7;
pub const DX_CMP_ALWAYS: i32 = 8;
pub const DX_ZCMP_DEFAULT: i32 = (DX_CMP_LESSEQUAL);
pub const DX_ZCMP_REVERSE: i32 = (DX_CMP_GREATEREQUAL);
pub const DX_SHADEMODE_FLAT: i32 = 1;
pub const DX_SHADEMODE_GOURAUD: i32 = 2;
pub const DX_FOGMODE_NONE: i32 = 0;
pub const DX_FOGMODE_EXP: i32 = 1;
pub const DX_FOGMODE_EXP2: i32 = 2;
pub const DX_FOGMODE_LINEAR: i32 = 3;
pub const DX_MATERIAL_TYPE_NORMAL: i32 = 0;
pub const DX_MATERIAL_TYPE_TOON: i32 = 1;
pub const DX_MATERIAL_TYPE_TOON_2: i32 = 2;
pub const DX_MATERIAL_BLENDTYPE_TRANSLUCENT: i32 = 0;
pub const DX_MATERIAL_BLENDTYPE_ADDITIVE: i32 = 1;
pub const DX_MATERIAL_BLENDTYPE_MODULATE: i32 = 2;
pub const DX_MATERIAL_BLENDTYPE_NONE: i32 = 3;
pub const DX_TEXADDRESS_WRAP: i32 = 1;
pub const DX_TEXADDRESS_MIRROR: i32 = 2;
pub const DX_TEXADDRESS_CLAMP: i32 = 3;
pub const DX_TEXADDRESS_BORDER: i32 = 4;
pub const DX_TEXADDRESS_NUM: i32 = 5;
pub const DX_SHADERTYPE_VERTEX: i32 = 0;
pub const DX_SHADERTYPE_PIXEL: i32 = 1;
pub const DX_SHADERTYPE_GEOMETRY: i32 = 2;
pub const DX_SHADERTYPE_COMPUTE: i32 = 3;
pub const DX_SHADERTYPE_DOMAIN: i32 = 4;
pub const DX_SHADERTYPE_HULL: i32 = 5;
pub const DX_VERTEX_TYPE_NORMAL_3D: i32 = 0;
pub const DX_VERTEX_TYPE_SHADER_3D: i32 = 1;
pub const DX_VERTEX_TYPE_NUM: i32 = 2;
pub const DX_INDEX_TYPE_16BIT: i32 = 0;
pub const DX_INDEX_TYPE_32BIT: i32 = 1;
pub const DX_LOADMODEL_PHYSICS_DISABLE: i32 = 1;
pub const DX_LOADMODEL_PHYSICS_LOADCALC: i32 = 0;
pub const DX_LOADMODEL_PHYSICS_REALTIME: i32 = 2;
pub const DX_LOADMODEL_PHYSICS_DISABLENAMEWORD_ALWAYS: i32 = 0;
pub const DX_LOADMODEL_PHYSICS_DISABLENAMEWORD_DISABLEPHYSICSFILEONLY: i32 = 1;
pub const DX_LOADMODEL_PHYSICS_DISABLENAMEWORD_NUM: i32 = 2;
pub const DX_SEMITRANSDRAWMODE_ALWAYS: i32 = 0;
pub const DX_SEMITRANSDRAWMODE_SEMITRANS_ONLY: i32 = 1;
pub const DX_SEMITRANSDRAWMODE_NOT_SEMITRANS_ONLY: i32 = 2;
pub const DX_CUBEMAP_FACE_POSITIVE_X: i32 = 0;
pub const DX_CUBEMAP_FACE_NEGATIVE_X: i32 = 1;
pub const DX_CUBEMAP_FACE_POSITIVE_Y: i32 = 2;
pub const DX_CUBEMAP_FACE_NEGATIVE_Y: i32 = 3;
pub const DX_CUBEMAP_FACE_POSITIVE_Z: i32 = 4;
pub const DX_CUBEMAP_FACE_NEGATIVE_Z: i32 = 5;
pub const DX_PRIMTYPE_POINTLIST: i32 = 1;
pub const DX_PRIMTYPE_LINELIST: i32 = 2;
pub const DX_PRIMTYPE_LINESTRIP: i32 = 3;
pub const DX_PRIMTYPE_TRIANGLELIST: i32 = 4;
pub const DX_PRIMTYPE_TRIANGLESTRIP: i32 = 5;
pub const DX_PRIMTYPE_TRIANGLEFAN: i32 = 6;
pub const DX_LIGHTTYPE_D3DLIGHT_POINT: i32 = 1;
pub const DX_LIGHTTYPE_D3DLIGHT_SPOT: i32 = 2;
pub const DX_LIGHTTYPE_D3DLIGHT_DIRECTIONAL: i32 = 3;
pub const DX_LIGHTTYPE_POINT: i32 = 1;
pub const DX_LIGHTTYPE_SPOT: i32 = 2;
pub const DX_LIGHTTYPE_DIRECTIONAL: i32 = 3;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_PAL4: i32 = 0;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_PAL8: i32 = 1;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_ALPHA_PAL4: i32 = 2;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_ALPHA_PAL8: i32 = 3;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_ALPHATEST_PAL4: i32 = 4;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_ALPHATEST_PAL8: i32 = 5;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_RGB16: i32 = 6;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_RGB32: i32 = 7;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_ALPHA_RGB16: i32 = 8;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_ALPHA_RGB32: i32 = 9;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_ALPHATEST_RGB16: i32 = 10;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_ALPHATEST_RGB32: i32 = 11;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_DXT1: i32 = 12;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_DXT2: i32 = 13;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_DXT3: i32 = 14;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_DXT4: i32 = 15;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_DXT5: i32 = 16;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_PLATFORM0: i32 = 17;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_PLATFORM1: i32 = 18;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_PLATFORM2: i32 = 19;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_PLATFORM3: i32 = 20;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_YUV: i32 = 21;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_ABGR_I16: i32 = 22;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_ABGR_F16: i32 = 23;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_ABGR_F32: i32 = 24;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_ONE_I8: i32 = 25;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_ONE_I16: i32 = 26;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_ONE_F16: i32 = 27;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_ONE_F32: i32 = 28;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_TWO_I8: i32 = 29;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_TWO_I16: i32 = 30;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_TWO_F16: i32 = 31;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_TWO_F32: i32 = 32;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_DRAWVALID_RGB16: i32 = 33;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_DRAWVALID_RGB32: i32 = 34;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_DRAWVALID_ALPHA_RGB32: i32 = 35;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_DRAWVALID_ABGR_I16: i32 = 36;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_DRAWVALID_ABGR_F16: i32 = 37;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_DRAWVALID_ABGR_F32: i32 = 38;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_DRAWVALID_ONE_I8: i32 = 39;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_DRAWVALID_ONE_I16: i32 = 40;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_DRAWVALID_ONE_F16: i32 = 41;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_DRAWVALID_ONE_F32: i32 = 42;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_DRAWVALID_TWO_I8: i32 = 43;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_DRAWVALID_TWO_I16: i32 = 44;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_DRAWVALID_TWO_F16: i32 = 45;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_DRAWVALID_TWO_F32: i32 = 46;
pub const DX_GRAPHICSIMAGE_FORMAT_3D_NUM: i32 = 47;
pub const DX_GRAPHICSIMAGE_FORMAT_2D: i32 = 48;
pub const DX_GRAPHICSIMAGE_FORMAT_R5G6B5: i32 = 49;
pub const DX_GRAPHICSIMAGE_FORMAT_X8A8R5G6B5: i32 = 50;
pub const DX_GRAPHICSIMAGE_FORMAT_X8R8G8B8: i32 = 51;
pub const DX_GRAPHICSIMAGE_FORMAT_A8R8G8B8: i32 = 52;
pub const DX_GRAPHICSIMAGE_FORMAT_NUM: i32 = 53;
pub const DX_BASEIMAGE_FORMAT_NORMAL: i32 = 0;
pub const DX_BASEIMAGE_FORMAT_DXT1: i32 = 1;
pub const DX_BASEIMAGE_FORMAT_DXT2: i32 = 2;
pub const DX_BASEIMAGE_FORMAT_DXT3: i32 = 3;
pub const DX_BASEIMAGE_FORMAT_DXT4: i32 = 4;
pub const DX_BASEIMAGE_FORMAT_DXT5: i32 = 5;
pub const DX_BASEIMAGE_FORMAT_PLATFORM0: i32 = 6;
pub const DX_BASEIMAGE_FORMAT_PLATFORM1: i32 = 7;
pub const DX_BASEIMAGE_FORMAT_PLATFORM2: i32 = 8;
pub const DX_BASEIMAGE_FORMAT_PLATFORM3: i32 = 9;
pub const DX_BASEIMAGE_FORMAT_YUV: i32 = 10;
pub const DX_WIN_ZTYPE_NORMAL: i32 = 0;
pub const DX_WIN_ZTYPE_BOTTOM: i32 = 1;
pub const DX_WIN_ZTYPE_TOP: i32 = 2;
pub const DX_WIN_ZTYPE_TOPMOST: i32 = 3;
pub const TOOLBUTTON_STATE_ENABLE: i32 = 0;
pub const TOOLBUTTON_STATE_PRESSED: i32 = 1;
pub const TOOLBUTTON_STATE_DISABLE: i32 = 2;
pub const TOOLBUTTON_STATE_PRESSED_DISABLE: i32 = 3;
pub const TOOLBUTTON_STATE_NUM: i32 = 4;
pub const TOOLBUTTON_TYPE_NORMAL: i32 = 0;
pub const TOOLBUTTON_TYPE_CHECK: i32 = 1;
pub const TOOLBUTTON_TYPE_GROUP: i32 = 2;
pub const TOOLBUTTON_TYPE_SEP: i32 = 3;
pub const TOOLBUTTON_TYPE_NUM: i32 = 4;
pub const MENUITEM_IDTOP: i32 = -1414812757;
pub const MENUITEM_ADD_CHILD: i32 = 0;
pub const MENUITEM_ADD_INSERT: i32 = 1;
pub const MENUITEM_MARK_NONE: i32 = 0;
pub const MENUITEM_MARK_CHECK: i32 = 1;
pub const MENUITEM_MARK_RADIO: i32 = 2;
pub const DX_NUMMODE_10: i32 = 0;
pub const DX_NUMMODE_16: i32 = 1;
pub const DX_STRMODE_NOT0: i32 = 2;
pub const DX_STRMODE_USE0: i32 = 3;
pub const DX_CHECKINPUT_KEY: i32 = 1;
pub const DX_CHECKINPUT_PAD: i32 = 2;
pub const DX_CHECKINPUT_MOUSE: i32 = 4;
pub const DX_CHECKINPUT_ALL: i32 = DX_CHECKINPUT_KEY;
pub const DX_INPUT_KEY_PAD1: i32 = 4097;
pub const DX_INPUT_PAD1: i32 = 1;
pub const DX_INPUT_PAD2: i32 = 2;
pub const DX_INPUT_PAD3: i32 = 3;
pub const DX_INPUT_PAD4: i32 = 4;
pub const DX_INPUT_PAD5: i32 = 5;
pub const DX_INPUT_PAD6: i32 = 6;
pub const DX_INPUT_PAD7: i32 = 7;
pub const DX_INPUT_PAD8: i32 = 8;
pub const DX_INPUT_PAD9: i32 = 9;
pub const DX_INPUT_PAD10: i32 = 10;
pub const DX_INPUT_PAD11: i32 = 11;
pub const DX_INPUT_PAD12: i32 = 12;
pub const DX_INPUT_PAD13: i32 = 13;
pub const DX_INPUT_PAD14: i32 = 14;
pub const DX_INPUT_PAD15: i32 = 15;
pub const DX_INPUT_PAD16: i32 = 16;
pub const DX_INPUT_KEY: i32 = 4096;
pub const DX_MOVIESURFACE_NORMAL: i32 = 0;
pub const DX_MOVIESURFACE_OVERLAY: i32 = 1;
pub const DX_MOVIESURFACE_FULLCOLOR: i32 = 2;
pub const TOUCHINPUTPOINT_MAX: i32 = 16;
pub const PAD_INPUT_DOWN: i32 = 1;
pub const PAD_INPUT_LEFT: i32 = 2;
pub const PAD_INPUT_RIGHT: i32 = 4;
pub const PAD_INPUT_UP: i32 = 8;
pub const PAD_INPUT_A: i32 = 16;
pub const PAD_INPUT_B: i32 = 32;
pub const PAD_INPUT_C: i32 = 64;
pub const PAD_INPUT_X: i32 = 128;
pub const PAD_INPUT_Y: i32 = 256;
pub const PAD_INPUT_Z: i32 = 512;
pub const PAD_INPUT_L: i32 = 1024;
pub const PAD_INPUT_R: i32 = 2048;
pub const PAD_INPUT_START: i32 = 4096;
pub const PAD_INPUT_M: i32 = 8192;
pub const PAD_INPUT_D: i32 = 16384;
pub const PAD_INPUT_F: i32 = 32768;
pub const PAD_INPUT_G: i32 = 65536;
pub const PAD_INPUT_H: i32 = 131072;
pub const PAD_INPUT_I: i32 = 262144;
pub const PAD_INPUT_J: i32 = 524288;
pub const PAD_INPUT_K: i32 = 1048576;
pub const PAD_INPUT_LL: i32 = 2097152;
pub const PAD_INPUT_N: i32 = 4194304;
pub const PAD_INPUT_O: i32 = 8388608;
pub const PAD_INPUT_P: i32 = 16777216;
pub const PAD_INPUT_RR: i32 = 33554432;
pub const PAD_INPUT_S: i32 = 67108864;
pub const PAD_INPUT_T: i32 = 134217728;
pub const PAD_INPUT_U: i32 = 268435456;
pub const PAD_INPUT_V: i32 = 536870912;
pub const PAD_INPUT_W: i32 = 1073741824;
pub const PAD_INPUT_XX: i32 = -2147483648;
pub const PAD_INPUT_1: i32 = 16;
pub const PAD_INPUT_2: i32 = 32;
pub const PAD_INPUT_3: i32 = 64;
pub const PAD_INPUT_4: i32 = 128;
pub const PAD_INPUT_5: i32 = 256;
pub const PAD_INPUT_6: i32 = 512;
pub const PAD_INPUT_7: i32 = 1024;
pub const PAD_INPUT_8: i32 = 2048;
pub const PAD_INPUT_9: i32 = 4096;
pub const PAD_INPUT_10: i32 = 8192;
pub const PAD_INPUT_11: i32 = 16384;
pub const PAD_INPUT_12: i32 = 32768;
pub const PAD_INPUT_13: i32 = 65536;
pub const PAD_INPUT_14: i32 = 131072;
pub const PAD_INPUT_15: i32 = 262144;
pub const PAD_INPUT_16: i32 = 524288;
pub const PAD_INPUT_17: i32 = 1048576;
pub const PAD_INPUT_18: i32 = 2097152;
pub const PAD_INPUT_19: i32 = 4194304;
pub const PAD_INPUT_20: i32 = 8388608;
pub const PAD_INPUT_21: i32 = 16777216;
pub const PAD_INPUT_22: i32 = 33554432;
pub const PAD_INPUT_23: i32 = 67108864;
pub const PAD_INPUT_24: i32 = 134217728;
pub const PAD_INPUT_25: i32 = 268435456;
pub const PAD_INPUT_26: i32 = 536870912;
pub const PAD_INPUT_27: i32 = 1073741824;
pub const PAD_INPUT_28: i32 = -2147483648;
pub const XINPUT_BUTTON_DPAD_UP: i32 = 0;
pub const XINPUT_BUTTON_DPAD_DOWN: i32 = 1;
pub const XINPUT_BUTTON_DPAD_LEFT: i32 = 2;
pub const XINPUT_BUTTON_DPAD_RIGHT: i32 = 3;
pub const XINPUT_BUTTON_START: i32 = 4;
pub const XINPUT_BUTTON_BACK: i32 = 5;
pub const XINPUT_BUTTON_LEFT_THUMB: i32 = 6;
pub const XINPUT_BUTTON_RIGHT_THUMB: i32 = 7;
pub const XINPUT_BUTTON_LEFT_SHOULDER: i32 = 8;
pub const XINPUT_BUTTON_RIGHT_SHOULDER: i32 = 9;
pub const XINPUT_BUTTON_A: i32 = 12;
pub const XINPUT_BUTTON_B: i32 = 13;
pub const XINPUT_BUTTON_X: i32 = 14;
pub const XINPUT_BUTTON_Y: i32 = 15;
pub const MOUSE_INPUT_LEFT: i32 = 1;
pub const MOUSE_INPUT_RIGHT: i32 = 2;
pub const MOUSE_INPUT_MIDDLE: i32 = 4;
pub const MOUSE_INPUT_1: i32 = 1;
pub const MOUSE_INPUT_2: i32 = 2;
pub const MOUSE_INPUT_3: i32 = 4;
pub const MOUSE_INPUT_4: i32 = 8;
pub const MOUSE_INPUT_5: i32 = 16;
pub const MOUSE_INPUT_6: i32 = 32;
pub const MOUSE_INPUT_7: i32 = 64;
pub const MOUSE_INPUT_8: i32 = 128;
pub const KEY_INPUT_BACK: i32 = 14;
pub const KEY_INPUT_TAB: i32 = 15;
pub const KEY_INPUT_RETURN: i32 = 28;
pub const KEY_INPUT_LSHIFT: i32 = 42;
pub const KEY_INPUT_RSHIFT: i32 = 54;
pub const KEY_INPUT_LCONTROL: i32 = 29;
pub const KEY_INPUT_RCONTROL: i32 = 157;
pub const KEY_INPUT_ESCAPE: i32 = 1;
pub const KEY_INPUT_SPACE: i32 = 57;
pub const KEY_INPUT_PGUP: i32 = 201;
pub const KEY_INPUT_PGDN: i32 = 209;
pub const KEY_INPUT_END: i32 = 207;
pub const KEY_INPUT_HOME: i32 = 199;
pub const KEY_INPUT_LEFT: i32 = 203;
pub const KEY_INPUT_UP: i32 = 200;
pub const KEY_INPUT_RIGHT: i32 = 205;
pub const KEY_INPUT_DOWN: i32 = 208;
pub const KEY_INPUT_INSERT: i32 = 210;
pub const KEY_INPUT_DELETE: i32 = 211;
pub const KEY_INPUT_MINUS: i32 = 12;
pub const KEY_INPUT_YEN: i32 = 125;
pub const KEY_INPUT_PREVTRACK: i32 = 144;
pub const KEY_INPUT_PERIOD: i32 = 52;
pub const KEY_INPUT_SLASH: i32 = 53;
pub const KEY_INPUT_LALT: i32 = 56;
pub const KEY_INPUT_RALT: i32 = 184;
pub const KEY_INPUT_SCROLL: i32 = 70;
pub const KEY_INPUT_SEMICOLON: i32 = 39;
pub const KEY_INPUT_COLON: i32 = 146;
pub const KEY_INPUT_LBRACKET: i32 = 26;
pub const KEY_INPUT_RBRACKET: i32 = 27;
pub const KEY_INPUT_AT: i32 = 145;
pub const KEY_INPUT_BACKSLASH: i32 = 43;
pub const KEY_INPUT_COMMA: i32 = 51;
pub const KEY_INPUT_KANJI: i32 = 148;
pub const KEY_INPUT_CONVERT: i32 = 121;
pub const KEY_INPUT_NOCONVERT: i32 = 123;
pub const KEY_INPUT_KANA: i32 = 112;
pub const KEY_INPUT_APPS: i32 = 221;
pub const KEY_INPUT_CAPSLOCK: i32 = 58;
pub const KEY_INPUT_SYSRQ: i32 = 183;
pub const KEY_INPUT_PAUSE: i32 = 197;
pub const KEY_INPUT_LWIN: i32 = 219;
pub const KEY_INPUT_RWIN: i32 = 220;
pub const KEY_INPUT_NUMLOCK: i32 = 69;
pub const KEY_INPUT_NUMPAD0: i32 = 82;
pub const KEY_INPUT_NUMPAD1: i32 = 79;
pub const KEY_INPUT_NUMPAD2: i32 = 80;
pub const KEY_INPUT_NUMPAD3: i32 = 81;
pub const KEY_INPUT_NUMPAD4: i32 = 75;
pub const KEY_INPUT_NUMPAD5: i32 = 76;
pub const KEY_INPUT_NUMPAD6: i32 = 77;
pub const KEY_INPUT_NUMPAD7: i32 = 71;
pub const KEY_INPUT_NUMPAD8: i32 = 72;
pub const KEY_INPUT_NUMPAD9: i32 = 73;
pub const KEY_INPUT_MULTIPLY: i32 = 55;
pub const KEY_INPUT_ADD: i32 = 78;
pub const KEY_INPUT_SUBTRACT: i32 = 74;
pub const KEY_INPUT_DECIMAL: i32 = 83;
pub const KEY_INPUT_DIVIDE: i32 = 181;
pub const KEY_INPUT_NUMPADENTER: i32 = 156;
pub const KEY_INPUT_F1: i32 = 59;
pub const KEY_INPUT_F2: i32 = 60;
pub const KEY_INPUT_F3: i32 = 61;
pub const KEY_INPUT_F4: i32 = 62;
pub const KEY_INPUT_F5: i32 = 63;
pub const KEY_INPUT_F6: i32 = 64;
pub const KEY_INPUT_F7: i32 = 65;
pub const KEY_INPUT_F8: i32 = 66;
pub const KEY_INPUT_F9: i32 = 67;
pub const KEY_INPUT_F10: i32 = 68;
pub const KEY_INPUT_F11: i32 = 87;
pub const KEY_INPUT_F12: i32 = 88;
pub const KEY_INPUT_A: i32 = 30;
pub const KEY_INPUT_B: i32 = 48;
pub const KEY_INPUT_C: i32 = 46;
pub const KEY_INPUT_D: i32 = 32;
pub const KEY_INPUT_E: i32 = 18;
pub const KEY_INPUT_F: i32 = 33;
pub const KEY_INPUT_G: i32 = 34;
pub const KEY_INPUT_H: i32 = 35;
pub const KEY_INPUT_I: i32 = 23;
pub const KEY_INPUT_J: i32 = 36;
pub const KEY_INPUT_K: i32 = 37;
pub const KEY_INPUT_L: i32 = 38;
pub const KEY_INPUT_M: i32 = 50;
pub const KEY_INPUT_N: i32 = 49;
pub const KEY_INPUT_O: i32 = 24;
pub const KEY_INPUT_P: i32 = 25;
pub const KEY_INPUT_Q: i32 = 16;
pub const KEY_INPUT_R: i32 = 19;
pub const KEY_INPUT_S: i32 = 31;
pub const KEY_INPUT_T: i32 = 20;
pub const KEY_INPUT_U: i32 = 22;
pub const KEY_INPUT_V: i32 = 47;
pub const KEY_INPUT_W: i32 = 17;
pub const KEY_INPUT_X: i32 = 45;
pub const KEY_INPUT_Y: i32 = 21;
pub const KEY_INPUT_Z: i32 = 44;
pub const KEY_INPUT_0: i32 = 11;
pub const KEY_INPUT_1: i32 = 2;
pub const KEY_INPUT_2: i32 = 3;
pub const KEY_INPUT_3: i32 = 4;
pub const KEY_INPUT_4: i32 = 5;
pub const KEY_INPUT_5: i32 = 6;
pub const KEY_INPUT_6: i32 = 7;
pub const KEY_INPUT_7: i32 = 8;
pub const KEY_INPUT_8: i32 = 9;
pub const KEY_INPUT_9: i32 = 10;
pub const CTRL_CODE_BS: i32 = 8;
pub const CTRL_CODE_TAB: i32 = 9;
pub const CTRL_CODE_CR: i32 = 13;
pub const CTRL_CODE_DEL: i32 = 16;
pub const CTRL_CODE_COPY: i32 = 3;
pub const CTRL_CODE_PASTE: i32 = 22;
pub const CTRL_CODE_CUT: i32 = 24;
pub const CTRL_CODE_ALL: i32 = 1;
pub const CTRL_CODE_LEFT: i32 = 29;
pub const CTRL_CODE_RIGHT: i32 = 28;
pub const CTRL_CODE_UP: i32 = 30;
pub const CTRL_CODE_DOWN: i32 = 31;
pub const CTRL_CODE_HOME: i32 = 26;
pub const CTRL_CODE_END: i32 = 25;
pub const CTRL_CODE_PAGE_UP: i32 = 23;
pub const CTRL_CODE_PAGE_DOWN: i32 = 21;
pub const CTRL_CODE_ESC: i32 = 27;
pub const CTRL_CODE_CMP: i32 = 32;
pub const DX_KEYINPSTRCOLOR_NORMAL_STR: i32 = 0;
pub const DX_KEYINPSTRCOLOR_NORMAL_STR_EDGE: i32 = 1;
pub const DX_KEYINPSTRCOLOR_NORMAL_CURSOR: i32 = 2;
pub const DX_KEYINPSTRCOLOR_SELECT_STR: i32 = 3;
pub const DX_KEYINPSTRCOLOR_SELECT_STR_EDGE: i32 = 4;
pub const DX_KEYINPSTRCOLOR_SELECT_STR_BACK: i32 = 5;
pub const DX_KEYINPSTRCOLOR_IME_STR: i32 = 6;
pub const DX_KEYINPSTRCOLOR_IME_STR_EDGE: i32 = 7;
pub const DX_KEYINPSTRCOLOR_IME_STR_BACK: i32 = 8;
pub const DX_KEYINPSTRCOLOR_IME_CURSOR: i32 = 9;
pub const DX_KEYINPSTRCOLOR_IME_LINE: i32 = 10;
pub const DX_KEYINPSTRCOLOR_IME_SELECT_STR: i32 = 11;
pub const DX_KEYINPSTRCOLOR_IME_SELECT_STR_EDGE: i32 = 12;
pub const DX_KEYINPSTRCOLOR_IME_SELECT_STR_BACK: i32 = 13;
pub const DX_KEYINPSTRCOLOR_IME_CONV_WIN_STR: i32 = 14;
pub const DX_KEYINPSTRCOLOR_IME_CONV_WIN_STR_EDGE: i32 = 15;
pub const DX_KEYINPSTRCOLOR_IME_CONV_WIN_SELECT_STR: i32 = 16;
pub const DX_KEYINPSTRCOLOR_IME_CONV_WIN_SELECT_STR_EDGE: i32 = 17;
pub const DX_KEYINPSTRCOLOR_IME_CONV_WIN_SELECT_STR_BACK: i32 = 18;
pub const DX_KEYINPSTRCOLOR_IME_CONV_WIN_EDGE: i32 = 19;
pub const DX_KEYINPSTRCOLOR_IME_CONV_WIN_BACK: i32 = 20;
pub const DX_KEYINPSTRCOLOR_IME_MODE_STR: i32 = 21;
pub const DX_KEYINPSTRCOLOR_IME_MODE_STR_EDGE: i32 = 22;
pub const DX_KEYINPSTRCOLOR_NUM: i32 = 23;
pub const DX_KEYINPSTR_ENDCHARAMODE_OVERWRITE: i32 = 0;
pub const DX_KEYINPSTR_ENDCHARAMODE_NOTCHANGE: i32 = 1;
pub const DX_FSRESOLUTIONMODE_DESKTOP: i32 = 0;
pub const DX_FSRESOLUTIONMODE_NATIVE: i32 = 1;
pub const DX_FSRESOLUTIONMODE_MAXIMUM: i32 = 2;
pub const DX_FSSCALINGMODE_BILINEAR: i32 = 0;
pub const DX_FSSCALINGMODE_NEAREST: i32 = 1;
pub const DX_CHANGESCREEN_OK: i32 = 0;
pub const DX_CHANGESCREEN_RETURN: i32 = 1;
pub const DX_CHANGESCREEN_DEFAULT: i32 = 2;
pub const DX_CHANGESCREEN_REFRESHNORMAL: i32 = 3;
pub const LOADIMAGE_TYPE_FILE: i32 = 0;
pub const LOADIMAGE_TYPE_MEM: i32 = 1;
pub const LOADIMAGE_TYPE_NONE: i32 = 1;
pub const HTTP_ERR_SERVER: i32 = 0;
pub const HTTP_ERR_NOTFOUND: i32 = 1;
pub const HTTP_ERR_MEMORY: i32 = 2;
pub const HTTP_ERR_LOST: i32 = 3;
pub const HTTP_ERR_NONE: i32 = 1;
pub const HTTP_RES_COMPLETE: i32 = 0;
pub const HTTP_RES_STOP: i32 = 1;
pub const HTTP_RES_ERROR: i32 = 2;
pub const HTTP_RES_NOW: i32 = 1;