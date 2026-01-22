
/*
LIST OF STOPS TO THEIR MIDI NUMBER

Pedal Swell to Pedal 0 0
Pedal Solo to Pedal 0 1
Solo Contra Viola 16 0 3
Solo Viole d'Orchestre 8 0 4
Solo Viole Celeste 8 0 5
Solo Viole Sourdine 8 0 6
Solo Viole Octaviante 4 0 7
Solo Cornet de Violes III 0 8
Solo Tuba 8 0 9
Solo Tuba Clarion 4 0 10
Solo Spare A Stop Line 0 11
Solo Spare B Stop Line 0 12
Solo Tremulant 0 13
Swell Tremulant 0 14
Choir Tremulant 0 15
Choir Sub Octave 0 16
Choir Unison Off 0 17
Choir Octave 0 18
Solo Harmonic Flute 8 0 19
Solo Concert Flute 4 0 20
Solo Harmonic Piccolo 2 0 21
Solo Orchestral Oboe 8 0 22
Solo Cor Anglais 8 0 23
Solo French Horn 8 0 24
Solo Orchestral Trumpet 8 0 25
Swell Solo to Swell 0 26
Solo Sub Octave 0 27
Solo Unison Off 0 28
Solo Octave 0 29
Swell Quintaten 16 0 30
Swell Open Diapason 8 0 31
Swell Violin Diapason 8 0 32
Swell Lieblich Gedackt 8 0 33
Swell Echo Gamba 8 0 34
Swell Voix Celestes 8 0 35
Swell Principal 4 0 36
Swell Lieblich Flute 4 0 37
Swell Twelfth 2 2/3 0 38
Swell Fifteenth 2 0 39
Swell Mixture V 0 40
Swell Contra Oboe 16 0 41
Swell Oboe 8 0 42
Swell Double Trumpet 16 0 43
Swell Trumpet 8 0 44
Swell Clarion 4 0 45

Swell Spare A Stop Line 0 46
Swell Spare B Stop Line 0 47
Great Solo to Great 0 48
Great Swell to Great 0 49
Great Choir to Great 0 50
Great Double Geigen 16 0 51
Great Open Diapason I 8 0 52
Great Open Diapason II 8 0 53
Great Open Diapason III 8 0 54
Great Geigen 8 0 55
Great Hohl Flute 8 0 56
Great Quint 5 1/3 0 57
Great Octave 4 0 58
Great Principal 4 0 59
Great Wald Flute 4 0 60
Great Octave Quint 2 2/3 0 61
Great Super Octave 2 0 62
Great Mixture III 0 63
Great Mixture V 0 64
Great Spare A Stop Line 0 65
Great Spare B Stop Line 0 66
Great Contra Tromba 16 0 67
Great Tromba 8 0 68
Great Octave Tromba 4 0 69
Swell Sub Octave 0 70
Swell Unison Off 0 71
Swell Octave 0 72
Choir Solo to Choir 0 73
Choir Swell to Choir 0 74
Choir Open Diapason 8 0 75
Choir Stopped Diapason 8 0 76
Choir Principal 4 0 77
Choir Stopped Flute 4 0 78
Choir Nazard 2 2/3 0 79
Choir Super Octave 2 0 80
Choir Tierce 1 3/5 0 81
Choir Larigot 1 1/3 0 82
Choir Twenty-Second 1 0 83
Choir Mixture III 0 84
Choir Spare Unenclosed Stop Line 0 85
Choir Double Dulciana 16 0 86
Choir Claribel Flute 8 0 87
Choir Salicional 8 0 88
Choir Vox Angelica 8 0 89
Choir Dulciana 8 0 90

Choir Dulcet 4 0 91
Choir Clarinet 8 0 92
Choir Cornopean 8 0 93
Choir Spare Enclosed Stop Line 0 94
Choir Contra Tromba 16 0 95
Choir Tromba 8 0 96
Choir Octave Tromba 4 0 97
Choir Tuba 8 0 98
Pedal Great to Pedal 0 101
Pedal Choir to Pedal 0 102
Pedal Double Open Wood 32 0 103
Pedal Open Metal 16 0 104
Pedal Open Wood I 16 0 105
Pedal Open Wood II 16 0 106
Pedal Violone 16 0 107
Pedal Bourdon 16 0 108
Pedal Quintaten 16 0 109
Pedal Viola 16 0 110
Pedal Dulciana 16 0 111
Pedal Octave Metal 8 0 112
Pedal Principal 8 0 113
Pedal Octave Wood 8 0 114
Pedal Flute 8 0 115
Pedal Octave Quint 5 1/3 0 116
Pedal Super Octave 4 0 117
Pedal Fifteenth 4 0 118
Pedal Octave Flute 4 0 119
Pedal Mixture IV 0 120
Pedal Double Ophicleide 32 0 121
Pedal Ophicleide 16 0 122
Pedal Trombone 16 0 123
Pedal Fagotto 16 0 124
Pedal Posaune 8 0 125
Pedal Octave Posaune 4 0 126
 */

use std::fmt::{Display, Formatter, Result};
use crate::organ::stops_table::OrganStop::PedalSwellToPedal;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OrganStop {
    PedalSwellToPedal = 0,
    PedalSoloToPedal = 1,
    // 2 intentionally unused
    SoloContraViola16 = 3,
    SoloVioleDOrchestre8 = 4,
    SoloVioleCeleste8 = 5,
    SoloVioleSourdine8 = 6,
    SoloVioleOctaviante4 = 7,
    SoloCornetDeViolesIII = 8,
    SoloTuba8 = 9,
    SoloTubaClarion4 = 10,
    SoloSpareAStopLine = 11,
    SoloSpareBStopLine = 12,
    SoloTremulant = 13,
    SwellTremulant = 14,
    ChoirTremulant = 15,
    ChoirSubOctave = 16,
    ChoirUnisonOff = 17,
    ChoirOctave = 18,
    SoloHarmonicFlute8 = 19,
    SoloConcertFlute4 = 20,
    SoloHarmonicPiccolo2 = 21,
    SoloOrchestralOboe8 = 22,
    SoloCorAnglais8 = 23,
    SoloFrenchHorn8 = 24,
    SoloOrchestralTrumpet8 = 25,
    SwellSoloToSwell = 26,
    SoloSubOctave = 27,
    SoloUnisonOff = 28,
    SoloOctave = 29,
    SwellQuintaten16 = 30,
    SwellOpenDiapason8 = 31,
    SwellViolinDiapason8 = 32,
    SwellLieblichGedackt8 = 33,
    SwellEchoGamba8 = 34,
    SwellVoixCelestes8 = 35,
    SwellPrincipal4 = 36,
    SwellLieblichFlute4 = 37,
    SwellTwelfth2_2_3 = 38,
    SwellFifteenth2 = 39,
    SwellMixtureV = 40,
    SwellContraOboe16 = 41,
    SwellOboe8 = 42,
    SwellDoubleTrumpet16 = 43,
    SwellTrumpet8 = 44,
    SwellClarion4 = 45,
    SwellSpareAStopLine = 46,
    SwellSpareBStopLine = 47,
    GreatSoloToGreat = 48,
    GreatSwellToGreat = 49,
    GreatChoirToGreat = 50,
    GreatDoubleGeigen16 = 51,
    GreatOpenDiapasonI8 = 52,
    GreatOpenDiapasonII8 = 53,
    GreatOpenDiapasonIII8 = 54,
    GreatGeigen8 = 55,
    GreatHohlFlute8 = 56,
    GreatQuint5_1_3 = 57,
    GreatOctave4 = 58,
    GreatPrincipal4 = 59,
    GreatWaldFlute4 = 60,
    GreatOctaveQuint2_2_3 = 61,
    GreatSuperOctave2 = 62,
    GreatMixtureIII = 63,
    GreatMixtureV = 64,
    GreatSpareAStopLine = 65,
    GreatSpareBStopLine = 66,
    GreatContraTromba16 = 67,
    GreatTromba8 = 68,
    GreatOctaveTromba4 = 69,
    SwellSubOctave = 70,
    SwellUnisonOff = 71,
    SwellOctave = 72,
    ChoirSoloToChoir = 73,
    ChoirSwellToChoir = 74,
    ChoirOpenDiapason8 = 75,
    ChoirStoppedDiapason8 = 76,
    ChoirPrincipal4 = 77,
    ChoirStoppedFlute4 = 78,
    ChoirNazard2_2_3 = 79,
    ChoirSuperOctave2 = 80,
    ChoirTierce1_3_5 = 81,
    ChoirLarigot1_1_3 = 82,
    ChoirTwentySecond1 = 83,
    ChoirMixtureIII = 84,
    ChoirSpareUnenclosedStopLine = 85,
    ChoirDoubleDulciana16 = 86,
    ChoirClaribelFlute8 = 87,
    ChoirSalicional8 = 88,
    ChoirVoxAngelica8 = 89,
    ChoirDulciana8 = 90,
    ChoirDulcet4 = 91,
    ChoirClarinet8 = 92,
    ChoirCornopean8 = 93,
    ChoirSpareEnclosedStopLine = 94,
    ChoirContraTromba16 = 95,
    ChoirTromba8 = 96,
    ChoirOctaveTromba4 = 97,
    ChoirTuba8 = 98,
    PedalGreatToPedal = 99,
    PedalChoirToPedal = 100,
    PedalDoubleOpenWood32 = 101,
    PedalOpenMetal16 = 102,
    PedalOpenWoodI16 = 103,
    PedalOpenWoodII16 = 104,
    PedalViolone16 = 105,
    PedalBourdon16 = 106,
    PedalQuintaten16 = 107,
    PedalViola16 = 108,
    PedalDulciana16 = 109,
    PedalOctaveMetal8 = 110,
    PedalPrincipal8 = 111,
    PedalOctaveWood8 = 112,
    PedalFlute8 = 113,
    PedalOctaveQuint5_1_3 = 114,
    PedalSuperOctave4 = 115,
    PedalFifteenth4 = 116,
    PedalOctaveFlute4 = 117,
    PedalMixtureIV = 118,
    PedalDoubleOphicleide32 = 119,
    PedalOphicleide16 = 120,
    PedalTrombone16 = 121,
    PedalFagotto16 = 122,
    PedalPosaune8 = 123,
    PedalOctavePosaune4 = 124,
}

impl Display for OrganStop {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let name = match self {
            OrganStop::PedalSwellToPedal => "Pedal Swell to Pedal",
            OrganStop::PedalSoloToPedal => "Pedal Solo to Pedal",
            OrganStop::SoloContraViola16 => "Solo Contra Viola 16",
            OrganStop::SoloVioleDOrchestre8 => "Solo Viole d'Orchestre 8",
            OrganStop::SoloVioleCeleste8 => "Solo Viole Celeste 8",
            OrganStop::SoloVioleSourdine8 => "Solo Viole Sourdine 8",
            OrganStop::SoloVioleOctaviante4 => "Solo Viole Octaviante 4",
            OrganStop::SoloCornetDeViolesIII => "Solo Cornet de Violes III",
            OrganStop::SoloTuba8 => "Solo Tuba 8",
            OrganStop::SoloTubaClarion4 => "Solo Tuba Clarion 4",
            OrganStop::SoloSpareAStopLine => "Solo Spare A Stop Line",
            OrganStop::SoloSpareBStopLine => "Solo Spare B Stop Line",
            OrganStop::SoloTremulant => "Solo Tremulant",
            OrganStop::SwellTremulant => "Swell Tremulant",
            OrganStop::ChoirTremulant => "Choir Tremulant",
            OrganStop::ChoirSubOctave => "Choir Sub Octave",
            OrganStop::ChoirUnisonOff => "Choir Unison Off",
            OrganStop::ChoirOctave => "Choir Octave",
            OrganStop::SoloHarmonicFlute8 => "Solo Harmonic Flute 8",
            OrganStop::SoloConcertFlute4 => "Solo Concert Flute 4",
            OrganStop::SoloHarmonicPiccolo2 => "Solo Harmonic Piccolo 2",
            OrganStop::SoloOrchestralOboe8 => "Solo Orchestral Oboe 8",
            OrganStop::SoloCorAnglais8 => "Solo Cor Anglais 8",
            OrganStop::SoloFrenchHorn8 => "Solo French Horn 8",
            OrganStop::SoloOrchestralTrumpet8 => "Solo Orchestral Trumpet 8",
            OrganStop::SwellSoloToSwell => "Swell Solo to Swell",
            OrganStop::SoloSubOctave => "Solo Sub Octave",
            OrganStop::SoloUnisonOff => "Solo Unison Off",
            OrganStop::SoloOctave => "Solo Octave",
            OrganStop::SwellQuintaten16 => "Swell Quintaten 16",
            OrganStop::SwellOpenDiapason8 => "Swell Open Diapason 8",
            OrganStop::SwellViolinDiapason8 => "Swell Violin Diapason 8",
            OrganStop::SwellLieblichGedackt8 => "Swell Lieblich Gedackt 8",
            OrganStop::SwellEchoGamba8 => "Swell Echo Gamba 8",
            OrganStop::SwellVoixCelestes8 => "Swell Voix Celestes 8",
            OrganStop::SwellPrincipal4 => "Swell Principal 4",
            OrganStop::SwellLieblichFlute4 => "Swell Lieblich Flute 4",
            OrganStop::SwellTwelfth2_2_3 => "Swell Twelfth 2 2/3",
            OrganStop::SwellFifteenth2 => "Swell Fifteenth 2",
            OrganStop::SwellMixtureV => "Swell Mixture V",
            OrganStop::SwellContraOboe16 => "Swell Contra Oboe 16",
            OrganStop::SwellOboe8 => "Swell Oboe 8",
            OrganStop::SwellDoubleTrumpet16 => "Swell Double Trumpet 16",
            OrganStop::SwellTrumpet8 => "Swell Trumpet 8",
            OrganStop::SwellClarion4 => "Swell Clarion 4",
            OrganStop::SwellSpareAStopLine => "Swell Spare A Stop Line",
            OrganStop::SwellSpareBStopLine => "Swell Spare B Stop Line",
            OrganStop::GreatSoloToGreat => "Great Solo to Great",
            OrganStop::GreatSwellToGreat => "Great Swell to Great",
            OrganStop::GreatChoirToGreat => "Great Choir to Great",
            OrganStop::GreatDoubleGeigen16 => "Great Double Geigen 16",
            OrganStop::GreatOpenDiapasonI8 => "Great Open Diapason I 8",
            OrganStop::GreatOpenDiapasonII8 => "Great Open Diapason II 8",
            OrganStop::GreatOpenDiapasonIII8 => "Great Open Diapason III 8",
            OrganStop::GreatGeigen8 => "Great Geigen 8",
            OrganStop::GreatHohlFlute8 => "Great Hohl Flute 8",
            OrganStop::GreatQuint5_1_3 => "Great Quint 5 1/3",
            OrganStop::GreatOctave4 => "Great Octave 4",
            OrganStop::GreatPrincipal4 => "Great Principal 4",
            OrganStop::GreatWaldFlute4 => "Great Wald Flute 4",
            OrganStop::GreatOctaveQuint2_2_3 => "Great Octave Quint 2 2/3",
            OrganStop::GreatSuperOctave2 => "Great Super Octave 2",
            OrganStop::GreatMixtureIII => "Great Mixture III",
            OrganStop::GreatMixtureV => "Great Mixture V",
            OrganStop::GreatSpareAStopLine => "Great Spare A Stop Line",
            OrganStop::GreatSpareBStopLine => "Great Spare B Stop Line",
            OrganStop::GreatContraTromba16 => "Great Contra Tromba 16",
            OrganStop::GreatTromba8 => "Great Tromba 8",
            OrganStop::GreatOctaveTromba4 => "Great Octave Tromba 4",
            OrganStop::SwellSubOctave => "Swell Sub Octave",
            OrganStop::SwellUnisonOff => "Swell Unison Off",
            OrganStop::SwellOctave => "Swell Octave",
            OrganStop::ChoirSoloToChoir => "Choir Solo to Choir",
            OrganStop::ChoirSwellToChoir => "Choir Swell to Choir",
            OrganStop::ChoirOpenDiapason8 => "Choir Open Diapason 8",
            OrganStop::ChoirStoppedDiapason8 => "Choir Stopped Diapason 8",
            OrganStop::ChoirPrincipal4 => "Choir Principal 4",
            OrganStop::ChoirStoppedFlute4 => "Choir Stopped Flute 4",
            OrganStop::ChoirNazard2_2_3 => "Choir Nazard 2 2/3",
            OrganStop::ChoirSuperOctave2 => "Choir Super Octave 2",
            OrganStop::ChoirTierce1_3_5 => "Choir Tierce 1 3/5",
            OrganStop::ChoirLarigot1_1_3 => "Choir Larigot 1 1/3",
            OrganStop::ChoirTwentySecond1 => "Choir Twenty-Second 1",
            OrganStop::ChoirMixtureIII => "Choir Mixture III",
            OrganStop::ChoirSpareUnenclosedStopLine => "Choir Spare Unenclosed Stop Line",
            OrganStop::ChoirDoubleDulciana16 => "Choir Double Dulciana 16",
            OrganStop::ChoirClaribelFlute8 => "Choir Claribel Flute 8",
            OrganStop::ChoirSalicional8 => "Choir Salicional 8",
            OrganStop::ChoirVoxAngelica8 => "Choir Vox Angelica 8",
            OrganStop::ChoirDulciana8 => "Choir Dulciana 8",
            OrganStop::ChoirDulcet4 => "Choir Dulcet 4",
            OrganStop::ChoirClarinet8 => "Choir Clarinet 8",
            OrganStop::ChoirCornopean8 => "Choir Cornopean 8",
            OrganStop::ChoirSpareEnclosedStopLine => "Choir Spare Enclosed Stop Line",
            OrganStop::ChoirContraTromba16 => "Choir Contra Tromba 16",
            OrganStop::ChoirTromba8 => "Choir Tromba 8",
            OrganStop::ChoirOctaveTromba4 => "Choir Octave Tromba 4",
            OrganStop::ChoirTuba8 => "Choir Tuba 8",
            OrganStop::PedalGreatToPedal => "Pedal Great to Pedal",
            OrganStop::PedalChoirToPedal => "Pedal Choir to Pedal",
            OrganStop::PedalDoubleOpenWood32 => "Pedal Double Open Wood 32",
            OrganStop::PedalOpenMetal16 => "Pedal Open Metal 16",
            OrganStop::PedalOpenWoodI16 => "Pedal Open Wood I 16",
            OrganStop::PedalOpenWoodII16 => "Pedal Open Wood II 16",
            OrganStop::PedalViolone16 => "Pedal Violone 16",
            OrganStop::PedalBourdon16 => "Pedal Bourdon 16",
            OrganStop::PedalQuintaten16 => "Pedal Quintaten 16",
            OrganStop::PedalViola16 => "Pedal Viola 16",
            OrganStop::PedalDulciana16 => "Pedal Dulciana 16",
            OrganStop::PedalOctaveMetal8 => "Pedal Octave Metal 8",
            OrganStop::PedalPrincipal8 => "Pedal Principal 8",
            OrganStop::PedalOctaveWood8 => "Pedal Octave Wood 8",
            OrganStop::PedalFlute8 => "Pedal Flute 8",
            OrganStop::PedalOctaveQuint5_1_3 => "Pedal Octave Quint 5 1/3",
            OrganStop::PedalSuperOctave4 => "Pedal Super Octave 4",
            OrganStop::PedalFifteenth4 => "Pedal Fifteenth 4",
            OrganStop::PedalOctaveFlute4 => "Pedal Octave Flute 4",
            OrganStop::PedalMixtureIV => "Pedal Mixture IV",
            OrganStop::PedalDoubleOphicleide32 => "Pedal Double Ophicleide 32",
            OrganStop::PedalOphicleide16 => "Pedal Ophicleide 16",
            OrganStop::PedalTrombone16 => "Pedal Trombone 16",
            OrganStop::PedalFagotto16 => "Pedal Fagotto 16",
            OrganStop::PedalPosaune8 => "Pedal Posaune 8",
            OrganStop::PedalOctavePosaune4 => "Pedal Octave Posaune 4",
        };
        write!(f, "{}", name)
    }
}