# Threat Model for delocaliser

This document defines the threat model for delocaliser, a privacy-preserving photo post-processing tool designed to defeat visual geolocation systems while maintaining human-perceived image quality.

## Goal

Post-process any image to defeat visual geolocation systems so the predicted location is coarser than 1 degree latitude/longitude (worse than city-level), while preserving human-perceived image quality. The output must withstand common post-processing (crop/resize/JPEG) and complete within 24 hours for 4K images on a 2022 desktop in the most aggressive mode.

## Attacker Classes

### 1. Retrieval-Based Geolocation

These systems match query images against large geotagged databases using learned embeddings.

**Representative systems:**
- CLIP-based retrieval models matching images to geotagged databases
- IM2GPS and successors using scene matching
- Commercial services like Picarta

**Attack characteristics:**
- High accuracy in areas with dense database coverage
- Vulnerable to perturbations that shift embedding space position
- Can be defeated by removing distinctive visual features

### 2. Classifier/Regressor Models (CNNs)

Deep learning models that directly predict geographic cells or coordinates from image pixels.

**Representative systems:**
- PlaNet (Google, 2016): Partitions Earth into S2 cells, classifies images into cells
- ISNs (Individual Scene Networks): Specialized per-scene classifiers
- GeoEstimation and successors

**Attack characteristics:**
- Learn subtle visual cues (vegetation, architecture, road markings)
- Robust to simple transformations
- Vulnerable to adversarial perturbations targeting specific layers

### 3. Vision-Language Models (VLMs)

Modern multimodal models that combine visual understanding with language reasoning for geolocation.

**Representative systems:**
- GPT-4V/GPT-4o (OpenAI)
- Claude 3.5 (Anthropic)
- Gemini 2.5 (Google)
- Specialized: PIGEON, GeoChat

**Attack characteristics:**
- Achieve geolocation performance on par with dedicated systems
- Use chain-of-thought reasoning to identify and combine multiple cues
- Can engage in conversational/iterative refinement (GptGeoChat-style attacks)
- Capable of cross-image correlation when multiple images are available
- May use external tools (search, maps) to verify hypotheses

**VLM-specific threats (from arXiv:2407.04952):**
- Belief update attacks: VLMs refine predictions through dialogue
- Cross-image correlation: Combining cues from multiple user images
- Tool-augmented geolocation: Using search engines and maps APIs

### 4. Hybrid/Ensemble Systems

Combinations of the above approaches for improved accuracy.

**Attack characteristics:**
- More robust than individual models
- Require multi-target adversarial optimization
- May include human-in-the-loop verification

## Attack Surfaces

### Pixel Content

Visual cues that reveal geographic information:

**Architecture:**
- Building styles (half-timbered, adobe, brutalist)
- Religious structures (minarets, pagodas, onion domes, steeples)
- Roof types and materials
- Window styles and shutters

**Vegetation:**
- Tree species (palm, eucalyptus, cherry blossom, baobab)
- Landscaping patterns
- Agricultural patterns
- Seasonal indicators

**Infrastructure:**
- Road markings and signage conventions
- Utility poles and power lines
- Traffic signals and street furniture
- Vehicle types (school buses, tuk-tuks)

**Signage and Text:**
- Street names and addresses
- Business names and logos
- License plates
- Language and script

**Environmental:**
- Sun angle and shadows
- Sky characteristics
- Terrain and geology
- Weather patterns

### Metadata

Non-pixel information embedded in image files:

**EXIF data:**
- GPS coordinates (latitude, longitude, altitude)
- Camera make/model (regional availability)
- Timestamps (timezone inference)
- Lens characteristics

**Other metadata:**
- IPTC location fields
- XMP geographic data
- Thumbnail with original GPS
- Editing software traces

### Compression Pipelines

Social media platforms apply transformations that may preserve or destroy protection:

| Platform | Resize | JPEG Quality | Other |
|----------|--------|--------------|-------|
| Twitter/X | 4096px max | Q85 | Strip EXIF |
| Instagram | 1080px max | Q80 | Strip EXIF |
| Facebook | 2048px max | Q85 | Strip EXIF |
| WhatsApp | 1600px max | Q70 | Heavy compression |
| Telegram | Optional | Q85 | Preserves if "as file" |

Protection must survive these transformations via Expectation-over-Transforms (EoT) training.

### Auxiliary Attack Vectors

**Geocoding APIs:**
- Even partial location hints can be refined via geocoding
- Business names + category can narrow to specific addresses

**Reverse image search:**
- Matching against indexed web images
- Street View matching

**Cross-image correlation:**
- Combining cues from multiple images posted by same user
- Temporal patterns (posting times, sequences)

## Privacy Tiers

Different use cases require different levels of protection. delocaliser supports granular privacy tiers:

| Privacy Tier | Max Accuracy @ 100km | Median Error | Target Use Case |
|--------------|---------------------|--------------|-----------------|
| Country | <5% | >500 km | Travel influencers, general sharing |
| City | <1% @ 25km | >100 km | General privacy-conscious users |
| Neighborhood | <0.1% @ 1km | >25 km | Privacy-sensitive individuals |
| Building | 0% @ 100m | >10 km | High-security, witness protection |

### Default Target (M0)

For initial development, we target the **Country** tier:
- Top-1 within 100 km: ≤5% against proxy geolocator suite
- Median error distance: ≥500 km (Haversine distance)

## Success Metrics and KPIs

### Privacy Efficacy

**Primary metrics:**
- Top-1 accuracy within R km (R = 1, 25, 100, 500, 2500 km)
- Median prediction error (Haversine distance)
- Distance error cumulative distribution function (ECDF)

**Benchmark targets:**
- IMAGEO-Bench style evaluation
- GeoChain reasoning task performance (when applicable)

**Measurement methodology:**
- Evaluate against ensemble of proxy geolocators
- Report metrics across multiple thresholds
- Include confidence intervals

### Visual Fidelity

**Primary metrics:**
- LPIPS (Learned Perceptual Image Patch Similarity): Target ≤0.06
- CIEDE2000 ΔE on non-edited regions: Within acceptable bounds (TBD)

**Secondary metrics:**
- SSIM (Structural Similarity Index)
- PSNR (Peak Signal-to-Noise Ratio)
- Human evaluation studies (future)

### Robustness

**Transform survival:**
- Random crop (up to 20%)
- Resize (0.5x to 2x)
- JPEG compression (Q=95, 85, 75)
- Combined transforms (simulating social media pipelines)

**Measurement:**
- Privacy metrics before and after transforms
- Degradation bounds

### Performance

**Processing time targets:**
- 4K image: ≤24h in Max mode on 2022 desktop
- 1080p image: ≤10 min in Fast mode

**Hardware baseline:**
- CPU: 8-core desktop processor (2022 vintage)
- GPU: Optional, NVIDIA RTX 3080 or equivalent
- RAM: 32 GB

## Logging and Telemetry Plan

### Development Telemetry

**Per-image metrics (logged during processing):**
```
{
  "image_id": "hash",
  "input_resolution": [width, height],
  "policy_preset": "Light|Balanced|Max",
  "stages_applied": ["exif", "text", "saliency", "inpaint", "perturb"],
  "processing_time_ms": {
    "total": 0,
    "per_stage": {}
  },
  "privacy_score_before": {},
  "privacy_score_after": {},
  "fidelity_metrics": {
    "lpips": 0.0,
    "ciede2000": 0.0
  }
}
```

**Aggregate metrics (for tuning):**
- Success rate by image category (indoor/outdoor, urban/rural)
- Processing time distributions
- Memory usage peaks

### Privacy Considerations for Telemetry

- No image content or derived features logged
- No GPS or location data logged
- Opt-in only for any telemetry
- Local-only by default

## Dataset Plan

### Evaluation Datasets

**Single-image geolocation:**
- IM2GPS3k: 3,000 geotagged Flickr images
- YFCC26k: 26,000 images from YFCC100M with verified coordinates
- IMAGEO-Bench: Recent benchmark for VLM geolocation

**Cross-view matching (for stress testing):**
- CVUSA: 35,000 ground-aerial pairs
- CVACT: 128,000 pairs with fine alignment
- VIGOR: 238,000 pairs with panoramas

**VLM-specific:**
- GptGeoChat: Conversational geolocation benchmark
- GptGeoChat-Synthetic: Synthetic extension

### Internal Test Set

**Composition:**
- 1,000 images minimum for development
- Stratified by: continent, scene type (urban/rural/indoor), difficulty
- Include edge cases: ambiguous locations, distinctive landmarks

**Sources (public, permissively licensed):**
- Wikimedia Commons (CC-licensed geotagged images)
- Mapillary (street-level, CC-BY-SA)
- OpenStreetCam (street-level, CC-BY-SA)

### Dataset Licensing

All datasets used must have licenses compatible with:
- Research use
- Derivative works (for publishing results)
- No requirement to share processed outputs

## Assumptions and Constraints

### In Scope

- Post-processing pipeline (not real-time capture)
- On-device/desktop processing with optional GPU
- Targeted content edits + adversarial perturbations
- Robust evaluation framework
- CLI and desktop UI

### Out of Scope

- Real-time capture integration
- Video processing
- Cloud API service
- Mobile app (initial release)

### Technical Assumptions

- Attackers have access to state-of-the-art geolocation models
- Attackers may use multiple models in ensemble
- Attackers may apply standard image transforms before analysis
- Attackers do not have access to original unprocessed image

### Threat Model Limitations

- Cannot protect against physical surveillance or other non-image channels
- Cannot protect if attacker has prior knowledge of approximate location
- Cannot guarantee protection against future unknown models
- Forensic detection of edits may be possible (see arXiv:2506.21002)

## References

### Core Geolocation Systems

1. PlaNet (arXiv:1602.05314): CNN-based global geolocation
2. PIGEON (CVPR 2024): State-of-art image geolocation
3. GeoEstimation: Hierarchical scene classification

### VLM Geolocation

4. arXiv:2407.04952: Granular privacy control for VLM geo-privacy
5. arXiv:2408.09474: VLM geolocation capabilities analysis

### Adversarial Methods

6. GeoShield (arXiv:2508.03209): Adversarial perturbations for VLM geo-privacy
7. Doubly-UAP (arXiv:2412.08108): Universal perturbations for VLMs
8. EoT (ICML 2018): Expectation over Transforms for robust adversarial examples

### Benchmarks

9. IMAGEO-Bench (arXiv:2508.01608): Single-image geolocation benchmark
10. GeoChain: Reasoning-based geolocation evaluation
11. arXiv:2406.09722: Comprehensive geolocation survey

### Visual Fidelity

12. LPIPS (arXiv:1801.03924): Learned perceptual similarity
13. CIEDE2000: Color difference formula

### Forensic Detection

14. arXiv:2506.21002: Inverse STR forensic detectability (~98% accuracy)

## Document History

| Version | Date | Changes |
|---------|------|---------|
| 0.1 | 2026-01-04 | Initial threat model for M0 |

## Acceptance Criteria (M0)

This document satisfies M0 acceptance criteria by providing:

- [x] Attacker classes: VLMs, CNNs, image-retrieval systems
- [x] Attack channels: Social media with compression/crops
- [x] Success metric thresholds: Privacy tiers with specific targets
- [x] Logging/telemetry plan: Structured logging schema
- [x] Dataset plan: Public/geotagged sources identified
- [x] KPIs: Measurable metrics for privacy, fidelity, robustness, performance
