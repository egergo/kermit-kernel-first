macro_rules! handle_irq_n {
    ($n:expr) => {
        unsafe {
            asm!("push 0" :::: "intel", "volatile");
            asm!("push $0" :: "i"($n) :: "intel", "volatile");
            asm!("jmp handle_irq" :::: "intel", "volatile");
            unreachable!();
        }
    }
}

// the ones with code:
// #DF 8
// #TS 10
// #NP 11
// #SS 12
// #GP 13
// #AC 17
// #SX 30
macro_rules! handle_irq_n_code {
    ($n:expr) => {
        unsafe {
            asm!("push $0" :: "i"($n) :: "intel", "volatile");
            asm!("jmp handle_irq" :::: "intel", "volatile");
            unreachable!();
        }
    }
}

// for i in `seq 0 255`; do echo "#[naked] fn handle_irq_${i}() -> ! { handle_irq_n!(${i}); }"; done
#[naked] pub extern "C" fn handle_irq_0() -> ! { handle_irq_n!(0); }
#[naked] pub extern "C" fn handle_irq_1() -> ! { handle_irq_n!(1); }
#[naked] pub extern "C" fn handle_irq_2() -> ! { handle_irq_n!(2); }
#[naked] pub extern "C" fn handle_irq_3() -> ! { handle_irq_n!(3); }
#[naked] pub extern "C" fn handle_irq_4() -> ! { handle_irq_n!(4); }
#[naked] pub extern "C" fn handle_irq_5() -> ! { handle_irq_n!(5); }
#[naked] pub extern "C" fn handle_irq_6() -> ! { handle_irq_n!(6); }
#[naked] pub extern "C" fn handle_irq_7() -> ! { handle_irq_n!(7); }
#[naked] pub extern "C" fn handle_irq_8() -> ! { handle_irq_n_code!(8); }
#[naked] pub extern "C" fn handle_irq_9() -> ! { handle_irq_n!(9); }
#[naked] pub extern "C" fn handle_irq_10() -> ! { handle_irq_n_code!(10); }
#[naked] pub extern "C" fn handle_irq_11() -> ! { handle_irq_n_code!(11); }
#[naked] pub extern "C" fn handle_irq_12() -> ! { handle_irq_n_code!(12); }
#[naked] pub extern "C" fn handle_irq_13() -> ! { handle_irq_n_code!(13); }
#[naked] pub extern "C" fn handle_irq_14() -> ! { handle_irq_n_code!(14); }
#[naked] pub extern "C" fn handle_irq_15() -> ! { handle_irq_n!(15); }
#[naked] pub extern "C" fn handle_irq_16() -> ! { handle_irq_n!(16); }
#[naked] pub extern "C" fn handle_irq_17() -> ! { handle_irq_n_code!(17); }
#[naked] pub extern "C" fn handle_irq_18() -> ! { handle_irq_n!(18); }
#[naked] pub extern "C" fn handle_irq_19() -> ! { handle_irq_n!(19); }
#[naked] pub extern "C" fn handle_irq_20() -> ! { handle_irq_n!(20); }
#[naked] pub extern "C" fn handle_irq_21() -> ! { handle_irq_n!(21); }
#[naked] pub extern "C" fn handle_irq_22() -> ! { handle_irq_n!(22); }
#[naked] pub extern "C" fn handle_irq_23() -> ! { handle_irq_n!(23); }
#[naked] pub extern "C" fn handle_irq_24() -> ! { handle_irq_n!(24); }
#[naked] pub extern "C" fn handle_irq_25() -> ! { handle_irq_n!(25); }
#[naked] pub extern "C" fn handle_irq_26() -> ! { handle_irq_n!(26); }
#[naked] pub extern "C" fn handle_irq_27() -> ! { handle_irq_n!(27); }
#[naked] pub extern "C" fn handle_irq_28() -> ! { handle_irq_n!(28); }
#[naked] pub extern "C" fn handle_irq_29() -> ! { handle_irq_n!(29); }
#[naked] pub extern "C" fn handle_irq_30() -> ! { handle_irq_n_code!(30); }
#[naked] pub extern "C" fn handle_irq_31() -> ! { handle_irq_n!(31); }
#[naked] pub extern "C" fn handle_irq_32() -> ! { handle_irq_n!(32); }
#[naked] pub extern "C" fn handle_irq_33() -> ! { handle_irq_n!(33); }
#[naked] pub extern "C" fn handle_irq_34() -> ! { handle_irq_n!(34); }
#[naked] pub extern "C" fn handle_irq_35() -> ! { handle_irq_n!(35); }
#[naked] pub extern "C" fn handle_irq_36() -> ! { handle_irq_n!(36); }
#[naked] pub extern "C" fn handle_irq_37() -> ! { handle_irq_n!(37); }
#[naked] pub extern "C" fn handle_irq_38() -> ! { handle_irq_n!(38); }
#[naked] pub extern "C" fn handle_irq_39() -> ! { handle_irq_n!(39); }
#[naked] pub extern "C" fn handle_irq_40() -> ! { handle_irq_n!(40); }
#[naked] pub extern "C" fn handle_irq_41() -> ! { handle_irq_n!(41); }
#[naked] pub extern "C" fn handle_irq_42() -> ! { handle_irq_n!(42); }
#[naked] pub extern "C" fn handle_irq_43() -> ! { handle_irq_n!(43); }
#[naked] pub extern "C" fn handle_irq_44() -> ! { handle_irq_n!(44); }
#[naked] pub extern "C" fn handle_irq_45() -> ! { handle_irq_n!(45); }
#[naked] pub extern "C" fn handle_irq_46() -> ! { handle_irq_n!(46); }
#[naked] pub extern "C" fn handle_irq_47() -> ! { handle_irq_n!(47); }
#[naked] pub extern "C" fn handle_irq_48() -> ! { handle_irq_n!(48); }
#[naked] pub extern "C" fn handle_irq_49() -> ! { handle_irq_n!(49); }
#[naked] pub extern "C" fn handle_irq_50() -> ! { handle_irq_n!(50); }
#[naked] pub extern "C" fn handle_irq_51() -> ! { handle_irq_n!(51); }
#[naked] pub extern "C" fn handle_irq_52() -> ! { handle_irq_n!(52); }
#[naked] pub extern "C" fn handle_irq_53() -> ! { handle_irq_n!(53); }
#[naked] pub extern "C" fn handle_irq_54() -> ! { handle_irq_n!(54); }
#[naked] pub extern "C" fn handle_irq_55() -> ! { handle_irq_n!(55); }
#[naked] pub extern "C" fn handle_irq_56() -> ! { handle_irq_n!(56); }
#[naked] pub extern "C" fn handle_irq_57() -> ! { handle_irq_n!(57); }
#[naked] pub extern "C" fn handle_irq_58() -> ! { handle_irq_n!(58); }
#[naked] pub extern "C" fn handle_irq_59() -> ! { handle_irq_n!(59); }
#[naked] pub extern "C" fn handle_irq_60() -> ! { handle_irq_n!(60); }
#[naked] pub extern "C" fn handle_irq_61() -> ! { handle_irq_n!(61); }
#[naked] pub extern "C" fn handle_irq_62() -> ! { handle_irq_n!(62); }
#[naked] pub extern "C" fn handle_irq_63() -> ! { handle_irq_n!(63); }
#[naked] pub extern "C" fn handle_irq_64() -> ! { handle_irq_n!(64); }
#[naked] pub extern "C" fn handle_irq_65() -> ! { handle_irq_n!(65); }
#[naked] pub extern "C" fn handle_irq_66() -> ! { handle_irq_n!(66); }
#[naked] pub extern "C" fn handle_irq_67() -> ! { handle_irq_n!(67); }
#[naked] pub extern "C" fn handle_irq_68() -> ! { handle_irq_n!(68); }
#[naked] pub extern "C" fn handle_irq_69() -> ! { handle_irq_n!(69); }
#[naked] pub extern "C" fn handle_irq_70() -> ! { handle_irq_n!(70); }
#[naked] pub extern "C" fn handle_irq_71() -> ! { handle_irq_n!(71); }
#[naked] pub extern "C" fn handle_irq_72() -> ! { handle_irq_n!(72); }
#[naked] pub extern "C" fn handle_irq_73() -> ! { handle_irq_n!(73); }
#[naked] pub extern "C" fn handle_irq_74() -> ! { handle_irq_n!(74); }
#[naked] pub extern "C" fn handle_irq_75() -> ! { handle_irq_n!(75); }
#[naked] pub extern "C" fn handle_irq_76() -> ! { handle_irq_n!(76); }
#[naked] pub extern "C" fn handle_irq_77() -> ! { handle_irq_n!(77); }
#[naked] pub extern "C" fn handle_irq_78() -> ! { handle_irq_n!(78); }
#[naked] pub extern "C" fn handle_irq_79() -> ! { handle_irq_n!(79); }
#[naked] pub extern "C" fn handle_irq_80() -> ! { handle_irq_n!(80); }
#[naked] pub extern "C" fn handle_irq_81() -> ! { handle_irq_n!(81); }
#[naked] pub extern "C" fn handle_irq_82() -> ! { handle_irq_n!(82); }
#[naked] pub extern "C" fn handle_irq_83() -> ! { handle_irq_n!(83); }
#[naked] pub extern "C" fn handle_irq_84() -> ! { handle_irq_n!(84); }
#[naked] pub extern "C" fn handle_irq_85() -> ! { handle_irq_n!(85); }
#[naked] pub extern "C" fn handle_irq_86() -> ! { handle_irq_n!(86); }
#[naked] pub extern "C" fn handle_irq_87() -> ! { handle_irq_n!(87); }
#[naked] pub extern "C" fn handle_irq_88() -> ! { handle_irq_n!(88); }
#[naked] pub extern "C" fn handle_irq_89() -> ! { handle_irq_n!(89); }
#[naked] pub extern "C" fn handle_irq_90() -> ! { handle_irq_n!(90); }
#[naked] pub extern "C" fn handle_irq_91() -> ! { handle_irq_n!(91); }
#[naked] pub extern "C" fn handle_irq_92() -> ! { handle_irq_n!(92); }
#[naked] pub extern "C" fn handle_irq_93() -> ! { handle_irq_n!(93); }
#[naked] pub extern "C" fn handle_irq_94() -> ! { handle_irq_n!(94); }
#[naked] pub extern "C" fn handle_irq_95() -> ! { handle_irq_n!(95); }
#[naked] pub extern "C" fn handle_irq_96() -> ! { handle_irq_n!(96); }
#[naked] pub extern "C" fn handle_irq_97() -> ! { handle_irq_n!(97); }
#[naked] pub extern "C" fn handle_irq_98() -> ! { handle_irq_n!(98); }
#[naked] pub extern "C" fn handle_irq_99() -> ! { handle_irq_n!(99); }
#[naked] pub extern "C" fn handle_irq_100() -> ! { handle_irq_n!(100); }
#[naked] pub extern "C" fn handle_irq_101() -> ! { handle_irq_n!(101); }
#[naked] pub extern "C" fn handle_irq_102() -> ! { handle_irq_n!(102); }
#[naked] pub extern "C" fn handle_irq_103() -> ! { handle_irq_n!(103); }
#[naked] pub extern "C" fn handle_irq_104() -> ! { handle_irq_n!(104); }
#[naked] pub extern "C" fn handle_irq_105() -> ! { handle_irq_n!(105); }
#[naked] pub extern "C" fn handle_irq_106() -> ! { handle_irq_n!(106); }
#[naked] pub extern "C" fn handle_irq_107() -> ! { handle_irq_n!(107); }
#[naked] pub extern "C" fn handle_irq_108() -> ! { handle_irq_n!(108); }
#[naked] pub extern "C" fn handle_irq_109() -> ! { handle_irq_n!(109); }
#[naked] pub extern "C" fn handle_irq_110() -> ! { handle_irq_n!(110); }
#[naked] pub extern "C" fn handle_irq_111() -> ! { handle_irq_n!(111); }
#[naked] pub extern "C" fn handle_irq_112() -> ! { handle_irq_n!(112); }
#[naked] pub extern "C" fn handle_irq_113() -> ! { handle_irq_n!(113); }
#[naked] pub extern "C" fn handle_irq_114() -> ! { handle_irq_n!(114); }
#[naked] pub extern "C" fn handle_irq_115() -> ! { handle_irq_n!(115); }
#[naked] pub extern "C" fn handle_irq_116() -> ! { handle_irq_n!(116); }
#[naked] pub extern "C" fn handle_irq_117() -> ! { handle_irq_n!(117); }
#[naked] pub extern "C" fn handle_irq_118() -> ! { handle_irq_n!(118); }
#[naked] pub extern "C" fn handle_irq_119() -> ! { handle_irq_n!(119); }
#[naked] pub extern "C" fn handle_irq_120() -> ! { handle_irq_n!(120); }
#[naked] pub extern "C" fn handle_irq_121() -> ! { handle_irq_n!(121); }
#[naked] pub extern "C" fn handle_irq_122() -> ! { handle_irq_n!(122); }
#[naked] pub extern "C" fn handle_irq_123() -> ! { handle_irq_n!(123); }
#[naked] pub extern "C" fn handle_irq_124() -> ! { handle_irq_n!(124); }
#[naked] pub extern "C" fn handle_irq_125() -> ! { handle_irq_n!(125); }
#[naked] pub extern "C" fn handle_irq_126() -> ! { handle_irq_n!(126); }
#[naked] pub extern "C" fn handle_irq_127() -> ! { handle_irq_n!(127); }
#[naked] pub extern "C" fn handle_irq_128() -> ! { handle_irq_n!(128); }
#[naked] pub extern "C" fn handle_irq_129() -> ! { handle_irq_n!(129); }
#[naked] pub extern "C" fn handle_irq_130() -> ! { handle_irq_n!(130); }
#[naked] pub extern "C" fn handle_irq_131() -> ! { handle_irq_n!(131); }
#[naked] pub extern "C" fn handle_irq_132() -> ! { handle_irq_n!(132); }
#[naked] pub extern "C" fn handle_irq_133() -> ! { handle_irq_n!(133); }
#[naked] pub extern "C" fn handle_irq_134() -> ! { handle_irq_n!(134); }
#[naked] pub extern "C" fn handle_irq_135() -> ! { handle_irq_n!(135); }
#[naked] pub extern "C" fn handle_irq_136() -> ! { handle_irq_n!(136); }
#[naked] pub extern "C" fn handle_irq_137() -> ! { handle_irq_n!(137); }
#[naked] pub extern "C" fn handle_irq_138() -> ! { handle_irq_n!(138); }
#[naked] pub extern "C" fn handle_irq_139() -> ! { handle_irq_n!(139); }
#[naked] pub extern "C" fn handle_irq_140() -> ! { handle_irq_n!(140); }
#[naked] pub extern "C" fn handle_irq_141() -> ! { handle_irq_n!(141); }
#[naked] pub extern "C" fn handle_irq_142() -> ! { handle_irq_n!(142); }
#[naked] pub extern "C" fn handle_irq_143() -> ! { handle_irq_n!(143); }
#[naked] pub extern "C" fn handle_irq_144() -> ! { handle_irq_n!(144); }
#[naked] pub extern "C" fn handle_irq_145() -> ! { handle_irq_n!(145); }
#[naked] pub extern "C" fn handle_irq_146() -> ! { handle_irq_n!(146); }
#[naked] pub extern "C" fn handle_irq_147() -> ! { handle_irq_n!(147); }
#[naked] pub extern "C" fn handle_irq_148() -> ! { handle_irq_n!(148); }
#[naked] pub extern "C" fn handle_irq_149() -> ! { handle_irq_n!(149); }
#[naked] pub extern "C" fn handle_irq_150() -> ! { handle_irq_n!(150); }
#[naked] pub extern "C" fn handle_irq_151() -> ! { handle_irq_n!(151); }
#[naked] pub extern "C" fn handle_irq_152() -> ! { handle_irq_n!(152); }
#[naked] pub extern "C" fn handle_irq_153() -> ! { handle_irq_n!(153); }
#[naked] pub extern "C" fn handle_irq_154() -> ! { handle_irq_n!(154); }
#[naked] pub extern "C" fn handle_irq_155() -> ! { handle_irq_n!(155); }
#[naked] pub extern "C" fn handle_irq_156() -> ! { handle_irq_n!(156); }
#[naked] pub extern "C" fn handle_irq_157() -> ! { handle_irq_n!(157); }
#[naked] pub extern "C" fn handle_irq_158() -> ! { handle_irq_n!(158); }
#[naked] pub extern "C" fn handle_irq_159() -> ! { handle_irq_n!(159); }
#[naked] pub extern "C" fn handle_irq_160() -> ! { handle_irq_n!(160); }
#[naked] pub extern "C" fn handle_irq_161() -> ! { handle_irq_n!(161); }
#[naked] pub extern "C" fn handle_irq_162() -> ! { handle_irq_n!(162); }
#[naked] pub extern "C" fn handle_irq_163() -> ! { handle_irq_n!(163); }
#[naked] pub extern "C" fn handle_irq_164() -> ! { handle_irq_n!(164); }
#[naked] pub extern "C" fn handle_irq_165() -> ! { handle_irq_n!(165); }
#[naked] pub extern "C" fn handle_irq_166() -> ! { handle_irq_n!(166); }
#[naked] pub extern "C" fn handle_irq_167() -> ! { handle_irq_n!(167); }
#[naked] pub extern "C" fn handle_irq_168() -> ! { handle_irq_n!(168); }
#[naked] pub extern "C" fn handle_irq_169() -> ! { handle_irq_n!(169); }
#[naked] pub extern "C" fn handle_irq_170() -> ! { handle_irq_n!(170); }
#[naked] pub extern "C" fn handle_irq_171() -> ! { handle_irq_n!(171); }
#[naked] pub extern "C" fn handle_irq_172() -> ! { handle_irq_n!(172); }
#[naked] pub extern "C" fn handle_irq_173() -> ! { handle_irq_n!(173); }
#[naked] pub extern "C" fn handle_irq_174() -> ! { handle_irq_n!(174); }
#[naked] pub extern "C" fn handle_irq_175() -> ! { handle_irq_n!(175); }
#[naked] pub extern "C" fn handle_irq_176() -> ! { handle_irq_n!(176); }
#[naked] pub extern "C" fn handle_irq_177() -> ! { handle_irq_n!(177); }
#[naked] pub extern "C" fn handle_irq_178() -> ! { handle_irq_n!(178); }
#[naked] pub extern "C" fn handle_irq_179() -> ! { handle_irq_n!(179); }
#[naked] pub extern "C" fn handle_irq_180() -> ! { handle_irq_n!(180); }
#[naked] pub extern "C" fn handle_irq_181() -> ! { handle_irq_n!(181); }
#[naked] pub extern "C" fn handle_irq_182() -> ! { handle_irq_n!(182); }
#[naked] pub extern "C" fn handle_irq_183() -> ! { handle_irq_n!(183); }
#[naked] pub extern "C" fn handle_irq_184() -> ! { handle_irq_n!(184); }
#[naked] pub extern "C" fn handle_irq_185() -> ! { handle_irq_n!(185); }
#[naked] pub extern "C" fn handle_irq_186() -> ! { handle_irq_n!(186); }
#[naked] pub extern "C" fn handle_irq_187() -> ! { handle_irq_n!(187); }
#[naked] pub extern "C" fn handle_irq_188() -> ! { handle_irq_n!(188); }
#[naked] pub extern "C" fn handle_irq_189() -> ! { handle_irq_n!(189); }
#[naked] pub extern "C" fn handle_irq_190() -> ! { handle_irq_n!(190); }
#[naked] pub extern "C" fn handle_irq_191() -> ! { handle_irq_n!(191); }
#[naked] pub extern "C" fn handle_irq_192() -> ! { handle_irq_n!(192); }
#[naked] pub extern "C" fn handle_irq_193() -> ! { handle_irq_n!(193); }
#[naked] pub extern "C" fn handle_irq_194() -> ! { handle_irq_n!(194); }
#[naked] pub extern "C" fn handle_irq_195() -> ! { handle_irq_n!(195); }
#[naked] pub extern "C" fn handle_irq_196() -> ! { handle_irq_n!(196); }
#[naked] pub extern "C" fn handle_irq_197() -> ! { handle_irq_n!(197); }
#[naked] pub extern "C" fn handle_irq_198() -> ! { handle_irq_n!(198); }
#[naked] pub extern "C" fn handle_irq_199() -> ! { handle_irq_n!(199); }
#[naked] pub extern "C" fn handle_irq_200() -> ! { handle_irq_n!(200); }
#[naked] pub extern "C" fn handle_irq_201() -> ! { handle_irq_n!(201); }
#[naked] pub extern "C" fn handle_irq_202() -> ! { handle_irq_n!(202); }
#[naked] pub extern "C" fn handle_irq_203() -> ! { handle_irq_n!(203); }
#[naked] pub extern "C" fn handle_irq_204() -> ! { handle_irq_n!(204); }
#[naked] pub extern "C" fn handle_irq_205() -> ! { handle_irq_n!(205); }
#[naked] pub extern "C" fn handle_irq_206() -> ! { handle_irq_n!(206); }
#[naked] pub extern "C" fn handle_irq_207() -> ! { handle_irq_n!(207); }
#[naked] pub extern "C" fn handle_irq_208() -> ! { handle_irq_n!(208); }
#[naked] pub extern "C" fn handle_irq_209() -> ! { handle_irq_n!(209); }
#[naked] pub extern "C" fn handle_irq_210() -> ! { handle_irq_n!(210); }
#[naked] pub extern "C" fn handle_irq_211() -> ! { handle_irq_n!(211); }
#[naked] pub extern "C" fn handle_irq_212() -> ! { handle_irq_n!(212); }
#[naked] pub extern "C" fn handle_irq_213() -> ! { handle_irq_n!(213); }
#[naked] pub extern "C" fn handle_irq_214() -> ! { handle_irq_n!(214); }
#[naked] pub extern "C" fn handle_irq_215() -> ! { handle_irq_n!(215); }
#[naked] pub extern "C" fn handle_irq_216() -> ! { handle_irq_n!(216); }
#[naked] pub extern "C" fn handle_irq_217() -> ! { handle_irq_n!(217); }
#[naked] pub extern "C" fn handle_irq_218() -> ! { handle_irq_n!(218); }
#[naked] pub extern "C" fn handle_irq_219() -> ! { handle_irq_n!(219); }
#[naked] pub extern "C" fn handle_irq_220() -> ! { handle_irq_n!(220); }
#[naked] pub extern "C" fn handle_irq_221() -> ! { handle_irq_n!(221); }
#[naked] pub extern "C" fn handle_irq_222() -> ! { handle_irq_n!(222); }
#[naked] pub extern "C" fn handle_irq_223() -> ! { handle_irq_n!(223); }
#[naked] pub extern "C" fn handle_irq_224() -> ! { handle_irq_n!(224); }
#[naked] pub extern "C" fn handle_irq_225() -> ! { handle_irq_n!(225); }
#[naked] pub extern "C" fn handle_irq_226() -> ! { handle_irq_n!(226); }
#[naked] pub extern "C" fn handle_irq_227() -> ! { handle_irq_n!(227); }
#[naked] pub extern "C" fn handle_irq_228() -> ! { handle_irq_n!(228); }
#[naked] pub extern "C" fn handle_irq_229() -> ! { handle_irq_n!(229); }
#[naked] pub extern "C" fn handle_irq_230() -> ! { handle_irq_n!(230); }
#[naked] pub extern "C" fn handle_irq_231() -> ! { handle_irq_n!(231); }
#[naked] pub extern "C" fn handle_irq_232() -> ! { handle_irq_n!(232); }
#[naked] pub extern "C" fn handle_irq_233() -> ! { handle_irq_n!(233); }
#[naked] pub extern "C" fn handle_irq_234() -> ! { handle_irq_n!(234); }
#[naked] pub extern "C" fn handle_irq_235() -> ! { handle_irq_n!(235); }
#[naked] pub extern "C" fn handle_irq_236() -> ! { handle_irq_n!(236); }
#[naked] pub extern "C" fn handle_irq_237() -> ! { handle_irq_n!(237); }
#[naked] pub extern "C" fn handle_irq_238() -> ! { handle_irq_n!(238); }
#[naked] pub extern "C" fn handle_irq_239() -> ! { handle_irq_n!(239); }
#[naked] pub extern "C" fn handle_irq_240() -> ! { handle_irq_n!(240); }
#[naked] pub extern "C" fn handle_irq_241() -> ! { handle_irq_n!(241); }
#[naked] pub extern "C" fn handle_irq_242() -> ! { handle_irq_n!(242); }
#[naked] pub extern "C" fn handle_irq_243() -> ! { handle_irq_n!(243); }
#[naked] pub extern "C" fn handle_irq_244() -> ! { handle_irq_n!(244); }
#[naked] pub extern "C" fn handle_irq_245() -> ! { handle_irq_n!(245); }
#[naked] pub extern "C" fn handle_irq_246() -> ! { handle_irq_n!(246); }
#[naked] pub extern "C" fn handle_irq_247() -> ! { handle_irq_n!(247); }
#[naked] pub extern "C" fn handle_irq_248() -> ! { handle_irq_n!(248); }
#[naked] pub extern "C" fn handle_irq_249() -> ! { handle_irq_n!(249); }
#[naked] pub extern "C" fn handle_irq_250() -> ! { handle_irq_n!(250); }
#[naked] pub extern "C" fn handle_irq_251() -> ! { handle_irq_n!(251); }
#[naked] pub extern "C" fn handle_irq_252() -> ! { handle_irq_n!(252); }
#[naked] pub extern "C" fn handle_irq_253() -> ! { handle_irq_n!(253); }
#[naked] pub extern "C" fn handle_irq_254() -> ! { handle_irq_n!(254); }
#[naked] pub extern "C" fn handle_irq_255() -> ! { handle_irq_n!(255); }
