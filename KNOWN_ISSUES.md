# Known Issues in Obadh Engine

This document tracks known issues and planned future work for the Obadh Engine.

## Tokenizer Issues

1. **Conjunct Formation**: Not all conjuncts are legal in Bengali. The current tokenizer attempts to form conjuncts too aggressively, which can lead to incorrect transliterations.

2. **'chhi' Handling**: The sequence "chhi" is incorrectly tokenized as `ch,,hi` (forming a conjunct), but in proper Bengali it should render as "ছি" (ছ + ি) without a conjunct. Similar issues exist with other aspirated consonants.

3. **Complex Rule Handling**: The tokenizer needs more sophisticated rules to handle special cases like:
   - When two consonants should form conjuncts vs. when they should remain separate
   - Proper handling of consonant clusters like "ksh", "jn", etc.

4. **Consonant Cluster Recognition**: The tokenizer does not correctly identify which consonant clusters should form conjuncts and which should remain separate. For example:
   - "str" in "strI" should be transliterated as "স্ত্র" but currently becomes "স্তর"
   - "kt" in "bhakt" should be transliterated as "ক্ত" but can be incorrectly handled

## Transliterator Issues

1. **Special Case Handling**: The transliterator needs better special case handling for sequences like:
   - "chhi" → "ছি" (currently transliterates to "ছ্হি")
   - "korchhi" → "করছি" (currently transliterates to "কর্ছহি")
   - The entire sequence "strI bhakt prokash korchhi" → "স্ত্রী ভক্ত প্রকাশ করছি" (current: "স্তরী ভাক্ত প্রকাশ কর্ছহি")

2. **Aspirated Consonant Handling**: Aspirated consonants like "chh", "jh", "th", "bh" are sometimes incorrectly processed, especially when followed by vowels.

3. **Heuristic Improvements**: Better heuristics needed for common Bengali words and patterns.

## Future Work

1. Implement a more linguistically accurate algorithm for forming conjuncts based on Bengali orthography rules.

2. Add special case handling for common word patterns.

3. Add a dictionary-based approach to supplement the algorithmic transliteration for common words.

4. Reduce tokenizer warnings by cleaning up unused code.

5. Expand test coverage to ensure all edge cases are handled correctly.

6. Add a phonetic rule system that better matches Bengali orthography's special cases.

7. Consider implementing a preprocessing step that identifies known problematic patterns before tokenization.

## Notes

The current version of the engine works well for basic transliteration cases like:
- "lal" → "লাল"
- "jhuTi" → "ঝুটি"
- "kakatuta" → "কাকাতুতা"
- "ami banglay gan gai" → "আমি বাংলায় গান গাই"

But more complex cases involving conjuncts and special character combinations need improvement.

## Specific Examples of Issues

| Input | Current Output | Expected Output | Issue |
|-------|---------------|-----------------|-------|
| "chhi" | ছ্হি | ছি | Incorrect conjunct formation |
| "korchhi" | কর্ছহি | করছি | Incorrect handling of "chhi" |
| "strI" | স্তরী | স্ত্রী | Incorrect consonant cluster handling |
| "bhakt" | ভাক্ত | ভক্ত | Unnecessary vowel insertion | 