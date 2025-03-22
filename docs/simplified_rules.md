# Simplified Transliteration Rules

This document outlines the core rules and special cases for Bengali transliteration in the Obadh Engine.

## Core Approach

This implementation takes a structured approach to Bengali transliteration, focusing on:

### 1. Vowel Handling

Vowels are handled in different modes based on their position and context:

- **Independent Vowels**: When vowels appear on their own
  - Example: `o` → `অ`, `O` → `ও`

- **Vowel Modifiers**: When vowels follow consonants
  - Example: `ka` → `কা`, `ki` → `কি`

- **Inherent Vowel**: The implied 'অ' sound after consonants
  - Example: `k` → `ক` (inherently includes the 'অ' sound)

### 2. Consonant Handling

Consonants are handled in different modes:

- **Independent**: Individual consonants with inherent vowel
  - Example: `k` → `ক`, `kh` → `খ`

- **With Vowel Modifiers**: Consonants followed by explicit vowels
  - Example: `ka` → `কা`, `ki` → `কি`

- **Conjuncts**: Multiple consonants combined with hasant
  - Example: `kk` → `ক্ক`, `kt` → `ক্ত`
  - Created by adding hasant (্) between consonants

### 3. Special Reph Form

- Double 'r' creates the special reph form
  - Example: `rrm` → `র্ম`
  - This is different from a normal sequence: `rm` → `রম`
  - Reph form is created by adding hasant (্) between র and the following consonant (e.g. `rrm` → `র্ম`, `rrk` → `র্ক`)

### 4. Numeric Characters

Bengali has its own numerals that are mapped directly from Latin numerals:

| Latin | Bengali |
|-------|---------|
| 0     | ০       |
| 1     | ১       |
| 2     | ২       |
| 3     | ৩       |
| 4     | ৪       |
| 5     | ৫       |
| 6     | ৬       |
| 7     | ৭       |
| 8     | ৮       |
| 9     | ৯       |

- Numerals are transliterated directly to their Bengali equivalents
- They do not participate in conjunct formation or vowel modification
- Example: `123` → `১২৩`, `k2` → `ক২`

## Special Rules

### Hasanta Handling

- Represented as `,` in Avro phonetic layout
  - Example: `k,,` → `ক্‌`
- When `,,` is followed by non-whitespace, it acts as "o" and terminates:
  - Conjunct formation (prevents the next consonant from forming a conjunct)
  - Vowel modification (prevents the next character from modifying the vowel)

### 'o' as a Blocker

The 'o' character serves special functions:
- Acts as conjunct blocker
  - Example: `kok` → `কক` (prevents 'k' from forming a conjunct with the following 'k')
- Acts as vowel-modifier blocker

### য-ফলা and ব-ফলা Handling

There's special handling for য-ফলা and ব-ফলা:

- **Regular consonants** use 'z' and 'b': 
  - `z` → `য` (ya)
  - `b` → `ব` (ba)

- **Phola forms** (conjunct versions) use 'y' and 'w':
  - `ky` → `ক্য` (k + hasant + ya)
  - `kw` → `ক্ব` (k + hasant + ba)

- **No conjunct formation** with 'z' and 'b':
  - `kz` → `কয` (k + ya, no conjunct)
  - `kb` → `কব` (k + ba, no conjunct) 