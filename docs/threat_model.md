# Threat Model for delocaliser

This document defines the threat model for **delocaliser**, a privacy-preserving photo post-processing tool designed to defeat visual geolocation systems. It establishes the adversarial context, attack surfaces, and design constraints that guide the system's architecture.

## Executive Summary

delocaliser processes images to prevent AI-based geolocators from accurately inferring location while maintaining human-perceived image quality. The system must defend against three classes of attackers: retrieval-based systems, classifier/regressor models, and reasoning-capable vision-language models (VLMs). Protection must survive common image transformations and social media compression pipelines.

## Attacker Classes

### 1. Retrieval-Based Geolocation

These systems match query images against large geotagged databases using learned embeddings.

**Characteristics:**
- Use CLIP-like models to compute image embeddings
- Match against databases of millions of geotagged images (e.g., YFCC100M, Google Street View)
- Can leverage both global scene features and local landmarks
- Often achieve high accuracy in well-photographed areas

**Representative Systems:**
- OpenAI CLIP with geotagged retrieval databases
- Google's PlaNet retrieval variants
- Academic systems like Im2GPS and its successors

**Attack Vector:** Semantic similarity in embedding space allows matching even when pixel-level differences exist.

### 2. Classifier/Regressor Models

These systems directly predict geographic coordinates or discrete geographic cells from image features.

**Characteristics:**
- CNN-based architectures trained on millions of geotagged images
- Output either continuous coordinates or probability distributions over geographic cells
- Can operate at multiple granularities (country, region, city, neighborhood)
- Often use hierarchical classification schemes

**Representative Systems:**
- PlaNet (arXiv:1602.05314) - Classifies images into ~26,000 geographic cells
- PIGEON (CVPR 2024) - State-of-the-art image geolocation
- ISNs (Individual Scene Networks) - Specialized for specific scene types

**Attack Vector:** Learned features correlate with geographic indicators (architecture, vegetation, signage, infrastructure).

### 3. Reasoning Vision-Language Models (VLMs)

These systems combine visual understanding with chain-of-thought reasoning to infer location.

**Characteristics:**
- Can read and interpret text in images (signs, license plates, business names)
- Perform multi-step reasoning combining visual cues with world knowledge
- Can leverage external tools (search engines, geocoding APIs)
- Capable of explaining their reasoning, making them harder to fool
- May use partial information to narrow down location progressively

**Representative Systems:**
- GPT-4o with vision capabilities
- Claude 3.5 Sonnet
- Gemini 2.5 Pro
- Specialized geolocation agents (GeoChain-style systems)

**Attack Vector:** Even if direct model output is moderated, users can feed extracted hints into geocoders. VLMs can identify subtle cues humans might miss.

## Attack Surfaces

### 1. Pixel Content

Visual cues embedded in the image content itself.

**Geographic Indicators:**
- **Architecture:** Building styles, construction materials, roof types, window designs
- **Vegetation:** Plant species, landscaping patterns, seasonal indicators
- **Infrastructure:** Road markings, traffic signs, utility poles, street furniture
- **Landmarks:** Recognizable buildings, monuments, natural features
- **Vehicles:** License plate formats, car models common to specific regions
- **People:** Clothing styles, uniforms, crowd density patterns
- **Sky/Weather:** Sun position, cloud patterns, atmospheric conditions
- **Terrain:** Topography, soil color, geological features

**Mitigation Approach:** Geo-saliency detection followed by content rewriting or adversarial perturbation.

### 2. Metadata

Non-visual information embedded in image files.

**EXIF Data:**
- GPS coordinates (latitude, longitude, altitude)
- Timestamp (can reveal timezone)
- Camera make/model (regional availability patterns)
- Lens information
- Software used for editing

**Other Metadata:**
- XMP data
- IPTC information
- Thumbnail images (may retain original metadata)
- File system timestamps

**Mitigation Approach:** Complete metadata scrubbing using ExifTool.

### 3. Scene Text

Readable text within the image.

**Text Types:**
- Street signs and road names
- Business names and storefronts
- License plates
- Billboards and advertisements
- Graffiti and murals
- Event posters and notices
- Product labels and packaging

**Languages and Scripts:**
- Language identification reveals region
- Script systems narrow down geography
- Even partial text can be geocoded

**Mitigation Approach:** Text detection (EAST) + OCR (Tesseract) + inpainting or masking.

**Forensic Consideration:** Inverse Scene Text Removal research (arXiv:2506.21002) shows STR methods leave detectable fingerprints with ~98% accuracy. Text removal edits may be forensically detectable, requiring combination with adversarial perturbations to mask artifacts.

### 4. Compression Pipelines

Social media and messaging platforms apply their own processing.

**Common Transformations:**
- JPEG recompression (quality levels vary by platform)
- Resolution downscaling
- Aspect ratio changes
- Color space conversions
- Metadata stripping (partial or complete)

**Platform-Specific Processing:**
- Twitter: Aggressive compression, strips most metadata
- Instagram: Recompression, resolution limits, aspect ratio constraints
- Facebook: Variable compression based on upload method
- WhatsApp: Heavy compression for bandwidth optimization

**Mitigation Approach:** Expectation over Transformation (EoT) training to ensure robustness across compression levels.

### 5. Auxiliary Tools

External services that can be combined with partial information.

**Geocoding Services:**
- Google Maps API
- OpenStreetMap Nominatim
- Mapbox Geocoding

**Search Engines:**
- Reverse image search (Google Images, TinEye)
- Text-based search for identified landmarks or businesses

**Specialized Databases:**
- Geograph (UK landscape photos)
- Wikimedia Commons geotagged images
- Flickr geotagged collections

**Mitigation Approach:** Reduce available evidence rather than just hiding coordinates. Even partial cues can be combined with auxiliary tools.

## Key Design Insight

> Even if model output is moderated, users can feed extracted hints into geocoders. delocaliser must reduce *available evidence*, not just hide coordinates.

This insight drives the multi-stage pipeline approach: metadata scrubbing alone is insufficient when visual cues remain. Content rewriting and adversarial perturbations must work together to reduce the information available to any attacker, human or machine.

## Threat Scenarios

### Scenario 1: Casual Privacy Violation

**Attacker:** Individual using publicly available tools
**Capability:** Access to consumer VLMs (ChatGPT, Claude), reverse image search
**Goal:** Identify location of a person from their social media photos
**Defense Priority:** High - most common threat

### Scenario 2: Targeted Stalking

**Attacker:** Determined individual with technical skills
**Capability:** Multiple geolocation tools, willingness to cross-reference information
**Goal:** Track specific individual's movements over time
**Defense Priority:** Critical - highest harm potential

### Scenario 3: Mass Surveillance

**Attacker:** State actor or large organization
**Capability:** Custom-trained models, large reference databases, significant compute
**Goal:** Geolocate images at scale for intelligence purposes
**Defense Priority:** Moderate - requires significant resources to counter

### Scenario 4: Journalistic Source Protection

**Attacker:** Adversarial government or organization
**Capability:** State-level resources, motivation to identify sources
**Goal:** Identify location of whistleblowers or journalists
**Defense Priority:** Critical - life-safety implications

## Success Criteria

### Privacy Efficacy

| Metric | Target | Measurement |
|--------|--------|-------------|
| Top-1 accuracy within 100 km | ≤5% | Against proxy geolocator suite |
| Median error distance | ≥500 km | Haversine distance from true location |
| Top-5 accuracy within 1 km | ≤1% | Against ensemble of models |

Metrics will be reported across standardized benchmarks including IMAGEO-Bench and GeoChain-style reasoning tasks.

### Visual Fidelity

| Metric | Target | Notes |
|--------|--------|-------|
| LPIPS | ≤0.06 | Perceptual similarity to original |
| CIEDE2000 ΔE | TBD | Color difference in non-edited regions |
| Human preference | >80% | A/B testing for naturalness |

### Robustness

Protection must survive common transformations:
- Random crop (up to 20% of image area)
- Resize (0.5x to 2.0x)
- JPEG compression (Q=95, Q=85, Q=75)
- Platform-specific pipelines (Twitter, Instagram, Facebook, WhatsApp)

Achieved through Expectation over Transformation (EoT) training methodology.

### Performance

| Mode | Resolution | Target Time | Hardware |
|------|------------|-------------|----------|
| Fast | 1080p | ≤10 minutes | 2022 desktop GPU |
| Balanced | 1080p | ≤30 minutes | 2022 desktop GPU |
| Max | 4K | ≤24 hours | 2022 desktop GPU |

## Assumptions and Limitations

### In-Scope

- Static images (JPEG, PNG, WebP)
- Single-image geolocation attacks
- Automated and semi-automated attackers
- Common social media distribution channels

### Out-of-Scope

- Video content (future work)
- Multi-image correlation attacks (e.g., tracking across photo series)
- Physical world modifications (e.g., wearing location-revealing clothing)
- Metadata in proprietary formats
- Real-time processing requirements

### Known Limitations

1. **Forensic Detectability:** Some modifications (especially text removal) may be forensically detectable. This is a privacy-forensics tradeoff.

2. **Semantic Preservation:** Heavy content rewriting may alter the semantic meaning of images in ways users find unacceptable.

3. **Novel Architectures:** New geolocation models may find features not covered by current defenses.

4. **Human Recognizability:** If a human can recognize a location, the image may still be geolocatable through manual analysis.

## References

| Paper | Relevance |
|-------|-----------|
| GeoShield (arXiv:2508.03209) | Adversarial perturbations for VLM geo-privacy |
| PIGEON (CVPR 2024) | State-of-art image geolocation baseline |
| PlaNet (arXiv:1602.05314) | Classic CNN geolocation approach |
| Granular Privacy Control (arXiv:2407.04952) | VLM moderation for geo-privacy |
| Doubly-UAP (arXiv:2412.08108) | Universal perturbations for VLMs |
| EoT (ICML 2018) | Robust adversarial examples |
| Grounded-SAM | Open-vocabulary detection + segmentation |
| Inverse STR (arXiv:2506.21002) | STR forensic detectability (~98% accuracy) |

## Document History

| Version | Date | Changes |
|---------|------|---------|
| 0.1.0 | 2026-01-16 | Initial threat model |

## Next Steps

This threat model informs the design of subsequent milestones:

- **M1 (Baselines & Evaluation Harness):** Implement metrics and benchmarks defined here
- **M2 (Metadata & Text Sanitization):** Address metadata and scene text attack surfaces
- **M3 (Geo-Feature Detection):** Target pixel content attack surface
- **M4-M5 (Rewriting & Perturbation):** Implement core defenses
- **M6 (Feedback Loop):** Validate against success criteria
- **M7 (Red-Team Testing):** Stress-test against threat scenarios
