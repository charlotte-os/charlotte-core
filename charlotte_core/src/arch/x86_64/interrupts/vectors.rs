use core::arch::global_asm;

// handlers
extern "C" {
    fn iv_32();
    fn iv_33();
    fn iv_34();
    fn iv_35();
    fn iv_36();
    fn iv_37();
    fn iv_38();
    fn iv_39();
    fn iv_40();
    fn iv_41();
    fn iv_42();
    fn iv_43();
    fn iv_44();
    fn iv_45();
    fn iv_46();
    fn iv_47();
    fn iv_48();
    fn iv_49();
    fn iv_50();
    fn iv_51();
    fn iv_52();
    fn iv_53();
    fn iv_54();
    fn iv_55();
    fn iv_56();
    fn iv_57();
    fn iv_58();
    fn iv_59();
    fn iv_60();
    fn iv_61();
    fn iv_62();
    fn iv_63();
    fn iv_64();
    fn iv_65();
    fn iv_66();
    fn iv_67();
    fn iv_68();
    fn iv_69();
    fn iv_70();
    fn iv_71();
    fn iv_72();
    fn iv_73();
    fn iv_74();
    fn iv_75();
    fn iv_76();
    fn iv_77();
    fn iv_78();
    fn iv_79();
    fn iv_80();
    fn iv_81();
    fn iv_82();
    fn iv_83();
    fn iv_84();
    fn iv_85();
    fn iv_86();
    fn iv_87();
    fn iv_88();
    fn iv_89();
    fn iv_90();
    fn iv_91();
    fn iv_92();
    fn iv_93();
    fn iv_94();
    fn iv_95();
    fn iv_96();
    fn iv_97();
    fn iv_98();
    fn iv_99();
    fn iv_100();
    fn iv_101();
    fn iv_102();
    fn iv_103();
    fn iv_104();
    fn iv_105();
    fn iv_106();
    fn iv_107();
    fn iv_108();
    fn iv_109();
    fn iv_110();
    fn iv_111();
    fn iv_112();
    fn iv_113();
    fn iv_114();
    fn iv_115();
    fn iv_116();
    fn iv_117();
    fn iv_118();
    fn iv_119();
    fn iv_120();
    fn iv_121();
    fn iv_122();
    fn iv_123();
    fn iv_124();
    fn iv_125();
    fn iv_126();
    fn iv_127();
    fn iv_128();
    fn iv_129();
    fn iv_130();
    fn iv_131();
    fn iv_132();
    fn iv_133();
    fn iv_134();
    fn iv_135();
    fn iv_136();
    fn iv_137();
    fn iv_138();
    fn iv_139();
    fn iv_140();
    fn iv_141();
    fn iv_142();
    fn iv_143();
    fn iv_144();
    fn iv_145();
    fn iv_146();
    fn iv_147();
    fn iv_148();
    fn iv_149();
    fn iv_150();
    fn iv_151();
    fn iv_152();
    fn iv_153();
    fn iv_154();
    fn iv_155();
    fn iv_156();
    fn iv_157();
    fn iv_158();
    fn iv_159();
    fn iv_160();
    fn iv_161();
    fn iv_162();
    fn iv_163();
    fn iv_164();
    fn iv_165();
    fn iv_166();
    fn iv_167();
    fn iv_168();
    fn iv_169();
    fn iv_170();
    fn iv_171();
    fn iv_172();
    fn iv_173();
    fn iv_174();
    fn iv_175();
    fn iv_176();
    fn iv_177();
    fn iv_178();
    fn iv_179();
    fn iv_180();
    fn iv_181();
    fn iv_182();
    fn iv_183();
    fn iv_184();
    fn iv_185();
    fn iv_186();
    fn iv_187();
    fn iv_188();
    fn iv_189();
    fn iv_190();
    fn iv_191();
    fn iv_192();
    fn iv_193();
    fn iv_194();
    fn iv_195();
    fn iv_196();
    fn iv_197();
    fn iv_198();
    fn iv_199();
    fn iv_200();
    fn iv_201();
    fn iv_202();
    fn iv_203();
    fn iv_204();
    fn iv_205();
    fn iv_206();
    fn iv_207();
    fn iv_208();
    fn iv_209();
    fn iv_210();
    fn iv_211();
    fn iv_212();
    fn iv_213();
    fn iv_214();
    fn iv_215();
    fn iv_216();
    fn iv_217();
    fn iv_218();
    fn iv_219();
    fn iv_220();
    fn iv_221();
    fn iv_222();
    fn iv_223();
    fn iv_224();
    fn iv_225();
    fn iv_226();
    fn iv_227();
    fn iv_228();
    fn iv_229();
    fn iv_230();
    fn iv_231();
    fn iv_232();
    fn iv_233();
    fn iv_234();
    fn iv_235();
    fn iv_236();
    fn iv_237();
    fn iv_238();
    fn iv_239();
    fn iv_240();
    fn iv_241();
    fn iv_242();
    fn iv_243();
    fn iv_244();
    fn iv_245();
    fn iv_246();
    fn iv_247();
    fn iv_248();
    fn iv_249();
    fn iv_250();
    fn iv_251();
    fn iv_252();
    fn iv_253();
    fn iv_254();
    fn iv_255();

}
// end of handlers
pub const IV_HANDLERS: [unsafe extern "C" fn(); 224] = [
    iv_32, iv_33, iv_34, iv_35, iv_36, iv_37, iv_38, iv_39, iv_40, iv_41, iv_42, iv_43, iv_44,
    iv_45, iv_46, iv_47, iv_48, iv_49, iv_50, iv_51, iv_52, iv_53, iv_54, iv_55, iv_56, iv_57,
    iv_58, iv_59, iv_60, iv_61, iv_62, iv_63, iv_64, iv_65, iv_66, iv_67, iv_68, iv_69, iv_70,
    iv_71, iv_72, iv_73, iv_74, iv_75, iv_76, iv_77, iv_78, iv_79, iv_80, iv_81, iv_82, iv_83,
    iv_84, iv_85, iv_86, iv_87, iv_88, iv_89, iv_90, iv_91, iv_92, iv_93, iv_94, iv_95, iv_96,
    iv_97, iv_98, iv_99, iv_100, iv_101, iv_102, iv_103, iv_104, iv_105, iv_106, iv_107, iv_108,
    iv_109, iv_110, iv_111, iv_112, iv_113, iv_114, iv_115, iv_116, iv_117, iv_118, iv_119, iv_120,
    iv_121, iv_122, iv_123, iv_124, iv_125, iv_126, iv_127, iv_128, iv_129, iv_130, iv_131, iv_132,
    iv_133, iv_134, iv_135, iv_136, iv_137, iv_138, iv_139, iv_140, iv_141, iv_142, iv_143, iv_144,
    iv_145, iv_146, iv_147, iv_148, iv_149, iv_150, iv_151, iv_152, iv_153, iv_154, iv_155, iv_156,
    iv_157, iv_158, iv_159, iv_160, iv_161, iv_162, iv_163, iv_164, iv_165, iv_166, iv_167, iv_168,
    iv_169, iv_170, iv_171, iv_172, iv_173, iv_174, iv_175, iv_176, iv_177, iv_178, iv_179, iv_180,
    iv_181, iv_182, iv_183, iv_184, iv_185, iv_186, iv_187, iv_188, iv_189, iv_190, iv_191, iv_192,
    iv_193, iv_194, iv_195, iv_196, iv_197, iv_198, iv_199, iv_200, iv_201, iv_202, iv_203, iv_204,
    iv_205, iv_206, iv_207, iv_208, iv_209, iv_210, iv_211, iv_212, iv_213, iv_214, iv_215, iv_216,
    iv_217, iv_218, iv_219, iv_220, iv_221, iv_222, iv_223, iv_224, iv_225, iv_226, iv_227, iv_228,
    iv_229, iv_230, iv_231, iv_232, iv_233, iv_234, iv_235, iv_236, iv_237, iv_238, iv_239, iv_240,
    iv_241, iv_242, iv_243, iv_244, iv_245, iv_246, iv_247, iv_248, iv_249, iv_250, iv_251, iv_252,
    iv_253, iv_254, iv_255,
];

global_asm! {
    include_str!("vectors.asm"),
}
