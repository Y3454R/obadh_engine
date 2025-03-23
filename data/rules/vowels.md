# Bengali Vowel System in Avro Phonetic

## 1. Basic Vowels (স্বরবর্ণ)

| Roman Input | Independent Vowel | Vowel Symbol (Kar) | Name |
|-------------|-------------------|-------------------|------|
| o | অ | - (inherent) | অ-কার (a-kar) |
| A | আ | া | আ-কার (aa-kar) |
| i | ই | ি | ই-কার (i-kar) |
| I | ঈ | ী | ঈ-কার (dirgho i-kar) |
| u | উ | ু | উ-কার (u-kar) |
| U | ঊ | ূ | ঊ-কার (dirgho u-kar) |
| e | এ | ে | এ-কার (e-kar) |
| OI | ঐ | ৈ | ঐ-কার (oi-kar) |
| O | ও | ো | ও-কার (o-kar) |
| OU | ঔ | ৌ | ঔ-কার (ou-kar) |
| rri | ঋ | ৃ | ঋ-কার (ri-kar) |

## 2. Basic Rules for Vowel Usage

### 2.1 Independent Vowels vs. Vowel Symbols

- **Independent vowels** are used at the beginning of a word or when a vowel appears independently
- **Vowel symbols (kars)** are used when the vowel follows a consonant

### 2.2 Examples of Usage

| Word Type | Roman Input | Bengali Output | Explanation |
|-----------|-------------|----------------|-------------|
| Vowel Initial | amar | আমার | 'a' becomes আ at beginning |
| Vowel Initial | ele | এলে | 'e' becomes এ at beginning |
| After Consonant | kori | করি | 'i' becomes ি after 'k' (ক) |
| After Consonant | tumi | তুমি | 'u' becomes ু after 't' (ত) |

## 3. Vowel 'o' as Conjunct Breaker

One of the most important special rules in Avro Phonetic is using the vowel 'o' to prevent conjunct formation:

| Typing Pattern | Bengali Result | Example | Explanation |
|----------------|----------------|---------|-------------|
| kk | ক্ক | চক্কর (chokkor) | Forms conjunct (k + hasant + k) |
| kok | কক | বকবক (bokbok) | Prevents conjunct by inserting 'o' |
| kOk | কোক | কোক (Coke) | Inserts the full o-kar vowel |

This is crucial when you need to represent two consecutive same letters without forming a conjunct. The vowel 'o' acts as a separator while being minimally pronounced in natural speech.

## 4. Special Vowel Rules

### 4.1 Vowel + Vowel Combinations

| Combination | Roman Input | Bengali Output |
|-------------|-------------|----------------|
| a + a | aa | আ |
| a + i | ai | আই |
| a + u | au | আউ |
| a + e | ae | আএ |
| a + o | ao | আও |
| i + a | ia | ইয়া |
| i + o | io | ইও |
| e + o | eo | এও |

> aa is a special case which is equivalent to the independent vowel আ (A) and aa-kaar 
### 4.2 Edge Cases and Exceptions

1. **Inherent 'a' Sound Elimination:**
   - To eliminate the inherent 'a' sound at the end of a word, use hasant (্)
   - Example: "kor" → কর্ (not কর) [hasant is noted in Avro as ',,']
   
2. **Silent/Half 'a' Sound:**
   - In some cases, the 'a' sound is pronounced halfway
   - No special notation in Avro, follows pronunciation rules
   

### 4.3 Vowel Modifications

| Modification | Roman Input | Bengali Output | Example |
|--------------|-------------|----------------|---------|
| Nasalization | vowel + ^ | vowel + ঁ | cha^d (চাঁদ) |
| Visarga | : | ঃ | du:kh (দুঃখ) |

## 4. Consonant + Vowel Combinations

The following examples show how vowels combine with consonants:

| Combination | Roman Input | Bengali Output | 
|-------------|-------------|----------------|
| ক + আ | ka | কা |
| ক + ি | ki | কি |
| ক + ী | kI | কী |
| ক + ু | ku | কু |
| ক + ূ | kU | কূ |
| ক + ে | ke | কে |
| ক + ৈ | kOI | কৈ |
| ক + ো | ko | কো |
| ক + ৌ | kOU | কৌ |
| ক + ৃ | krri | কৃ |