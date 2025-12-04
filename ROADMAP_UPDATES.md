# Roadmap Updates Based on Literature Review

This document proposes updates to the Photo Delocalizer roadmap (issue #1) based on systematic literature review of 80+ scholarly sources. Each milestone has been analyzed against recent research findings to identify improvements, new techniques, and refined approaches.

---

## M0 — Project Kickoff & Threat Model

### Key Findings from Literature

**VLM Geolocation Threat (Mendes et al. 2024, arXiv:2407.04952):**
The GptGeoChat benchmark demonstrates that current VLMs (GPT-4V, LLaVA) achieve image geolocation performance on par with state-of-the-art dedicated systems. This confirms VLMs as a primary threat vector requiring explicit modeling. The paper introduces granular privacy levels (country, city, neighborhood, building) that should inform our threat model's success criteria.

**Granular Privacy Levels:**
- Country-level: Coarsest protection (>1000 km error)
- City-level: Target for "Light" preset (~100-500 km error)
- Neighborhood-level: Target for "Balanced" preset (~10-100 km error)
- Building-level: Target for "Max" preset (prevent any sub-city inference)

### Proposed Updates to M0

1. **Expand threat model to explicitly include VLM-based geolocation:**
   - Add GPT-4V, Claude, Gemini as threat actors alongside CNN-based systems
   - Model conversational/iterative geolocation attacks where VLMs refine predictions through dialogue
   - Include "belief update" attacks where partial information is combined across multiple images

2. **Add granular privacy tiers to success criteria:**
   ```
   Privacy Tier | Max Accuracy | Median Error | Use Case
   -------------|--------------|--------------|----------
   Country      | <5% @ 100km  | >500 km      | Travel influencers
   City         | <1% @ 25km   | >100 km      | General users
   Neighborhood | <0.1% @ 1km  | >25 km       | Privacy-sensitive
   Building     | 0% @ 100m    | >10 km       | High-security
   ```

3. **Add VLM-specific attack scenarios:**
   - Single-image geolocation via direct query
   - Multi-turn dialogue refinement (GptGeoChat-style)
   - Cross-image correlation attacks
   - Metadata + visual cue combination attacks

**New references for M0:**
- [31]: arXiv:2407.04952 "Granular Privacy Control for Geolocation with Vision Language Models"
- [32]: arXiv:2408.09474 "Image-Based Geolocation Using Large Vision-Language Models"
- [33]: arXiv:1912.02085 "Protecting Geolocation Privacy of Photo Collections"

---

## M1 — Baselines & Evaluation Harness

### Key Findings from Literature

**Cross-View Geo-localization Survey (Durgam et al. 2024, arXiv:2406.09722):**
Comprehensive survey identifies key evaluation paradigms:
- Siamese networks for learning similarity metrics between image pairs
- Transformer-based methods achieving SOTA on CVUSA/CVACT benchmarks
- Orientation encoding as additional input channel improves accuracy significantly
- Part-based representation learning for handling viewpoint variations

**Benchmark Datasets:**
- CVUSA: 35,532 ground-aerial pairs across USA
- CVACT: 128,334 pairs from Canberra, Australia
- VIGOR: 238,696 pairs with orientation labels
- Pittsburgh 250k: Urban street-level imagery

### Proposed Updates to M1

1. **Expand proxy geolocator suite:**
   - Add transformer-based geolocators (ViT-based retrieval)
   - Include orientation-aware models as stronger baselines
   - Add cross-view matching models (ground-to-aerial) for comprehensive coverage

2. **Add new benchmark datasets:**
   ```python
   BENCHMARKS = {
       "single_image": ["IMAGEO-Bench", "IM2GPS3k", "YFCC26k"],
       "cross_view": ["CVUSA", "CVACT", "VIGOR"],
       "reasoning": ["GeoChain", "GptGeoChat"],
       "vlm_specific": ["GptGeoChat-Synthetic"]
   }
   ```

3. **Implement additional evaluation metrics:**
   - Top-k accuracy at multiple radii: {1km, 25km, 100km, 500km, 1000km}
   - Distance ECDF (Empirical Cumulative Distribution Function)
   - GeoAccuracy@k for retrieval-based methods
   - Reasoning chain accuracy for VLM-based attacks

4. **Add VLM-specific evaluation:**
   - Conversational geolocation success rate
   - Belief update resistance (multi-turn attacks)
   - Cross-modal consistency (image + text attacks)

**New references for M1:**
- [41]: arXiv:2406.09722 "Cross-view geo-localization: a survey"
- [40]: arXiv:2203.16291 "AmsterTime: Visual Place Recognition Benchmark"
- [46]: arXiv:2412.06781 "Around the World in 80 Timesteps"

---

## M2 — Metadata & Obvious-Cue Sanitization

### Key Findings from Literature

**Selective Scene Text Removal (Mitani et al. 2023, arXiv:2309.00410):**
Introduces selective text removal that can target specific words while preserving others. Key innovations:
- Multi-module U-Net structure: background extraction → text extraction → selective removal → reconstruction
- Conditioned U-Net allows specifying target words at test time
- No explicit word recognition module needed
- Reduces target word recall from 88% to 25%

**EXIF as Privacy Vector (arXiv:2301.04647):**
EXIF metadata contains rich location signals beyond GPS:
- Camera model + lens can narrow location (regional availability)
- Timestamp + sun position enables geolocation
- White balance settings correlate with lighting conditions/regions

### Proposed Updates to M2

1. **Expand EXIF sanitization beyond GPS:**
   ```python
   EXIF_PRIVACY_TAGS = {
       "critical": ["-gps:all", "-xmp:geotag*"],
       "high": ["-exif:datetimeoriginal", "-exif:make", "-exif:model"],
       "medium": ["-exif:lensmodel", "-exif:focallength"],
       "low": ["-exif:whitebalance", "-exif:colorspace"]
   }
   ```

2. **Implement selective text removal (SSTR):**
   - Target geo-revealing text: street names, business names, license plates
   - Preserve non-geo text: generic signs, decorative text
   - Use multi-module architecture for efficient training
   - Add language-specific text detection for international coverage

3. **Add text classification for geo-relevance:**
   ```python
   GEO_TEXT_CATEGORIES = {
       "high_risk": ["street_name", "city_name", "business_name", "license_plate"],
       "medium_risk": ["phone_number", "postal_code", "transit_sign"],
       "low_risk": ["generic_sign", "brand_logo", "decorative"]
   }
   ```

4. **Implement cascaded text processing:**
   - Stage 1: EAST/DBNet for text detection
   - Stage 2: Recognition + geo-relevance classification
   - Stage 3: Selective removal or replacement
   - Stage 4: Inpainting with context-aware fill

**New references for M2:**
- [49]: arXiv:2309.00410 "Selective Scene Text Removal"
- [47]: arXiv:2301.04647 "EXIF as Language"
- [50]: arXiv:2011.09768 "Scene text removal via cascaded detection"

---

## M3 — Geo-Feature Detection & Masking

### Key Findings from Literature

**SAM Surveys (arXiv:2305.08196, arXiv:2306.06211):**
- SAM achieves strong zero-shot segmentation across domains
- Prompt engineering critical for optimal performance
- SAM 2 adds video support and improved efficiency
- Vulnerabilities to adversarial attacks identified (Attack-SAM)

**Grounding DINO 1.5 (arXiv:2405.10300):**
- Edge version optimized for deployment
- Improved open-set detection accuracy
- Better handling of fine-grained categories

**Deep Saliency (arXiv:1801.04261, arXiv:2109.01980):**
- Networks learn hierarchical saliency features
- Can be used to identify visually distinctive regions
- Saliency prior helps reduce visual distraction

### Proposed Updates to M3

1. **Expand geo-cue prompt library:**
   ```python
   GEO_PROMPTS = {
       "architecture": [
           "minaret", "pagoda", "onion dome", "half-timbered house",
           "adobe building", "colonial architecture", "brutalist building"
       ],
       "vegetation": [
           "saguaro cactus", "palm tree", "cherry blossom", "baobab tree",
           "eucalyptus", "bamboo forest", "mangrove"
       ],
       "infrastructure": [
           "blue EU license plate", "yellow US school bus", "red UK phone booth",
           "Japanese vending machine", "German autobahn sign"
       ],
       "natural": [
           "red rock formation", "white sand beach", "volcanic landscape",
           "fjord", "rice terrace", "savanna"
       ]
   }
   ```

2. **Implement hierarchical saliency ranking:**
   - Level 1: CLIP gradient-based saliency
   - Level 2: Occlusion sensitivity analysis
   - Level 3: VLM attention visualization
   - Combine scores for final geo-saliency ranking

3. **Add robustness to SAM attacks:**
   - Implement ensemble of segmentation models
   - Add perturbation-aware segmentation
   - Validate masks against multiple prompt variations

4. **Implement landmark recognition integration:**
   - Use landmark retrieval to identify known geo-cues
   - Cross-reference with geo-databases (OpenStreetMap, Google Landmarks)
   - Prioritize removal of recognized landmarks

**New references for M3:**
- [55]: arXiv:2305.08196 "Comprehensive Survey on SAM"
- [57]: arXiv:2405.10300 "Grounding DINO 1.5"
- [64]: arXiv:2305.00866 "Attack-SAM"

---

## M4 — Content Rewriting/Inpainting

### Key Findings from Literature

**RePaint (arXiv:2201.09865):**
- Denoising diffusion for inpainting without task-specific training
- Iterative resampling improves coherence
- Works with arbitrary mask shapes

**DiffEdit (arXiv:2210.11427):**
- Automatic mask generation from text prompts
- Semantic editing without manual annotation
- Preserves unedited regions naturally

**Multimodal Image Editing Survey (arXiv:2406.14555):**
- Text-guided editing enables semantic control
- Attention manipulation for localized edits
- Quality metrics: FID, LPIPS, CLIP-Score, user studies

### Proposed Updates to M4

1. **Implement multi-stage inpainting pipeline:**
   ```
   Stage 1: Coarse fill (large regions, structural coherence)
   Stage 2: Fine refinement (texture, details)
   Stage 3: Harmonization (color, lighting consistency)
   Stage 4: Anti-geo verification (ensure no new geo-cues introduced)
   ```

2. **Add negative prompting for geo-neutrality:**
   ```python
   NEGATIVE_PROMPTS = [
       "landmark", "famous building", "recognizable location",
       "street sign", "license plate", "distinctive architecture",
       "regional vegetation", "cultural symbol"
   ]
   ```

3. **Implement semantic consistency verification:**
   - Pre/post CLIP embedding similarity
   - Caption consistency check (VLM-generated)
   - Object preservation verification
   - Scene coherence scoring

4. **Add quality metrics beyond LPIPS:**
   - CIEDE2000 ΔE for color fidelity
   - FID for distribution matching
   - CLIP-IQA for perceptual quality
   - Human preference prediction (HPD v2)

**New references for M4:**
- [65]: arXiv:2201.09865 "RePaint"
- [67]: arXiv:2210.11427 "DiffEdit"
- [69]: arXiv:2406.14555 "Survey of Multimodal-Guided Image Editing"

---

## M5 — Adversarial Perturbation Engine

### Key Findings from Literature

**Doubly-Universal Adversarial Perturbations (Kim et al. 2024, arXiv:2412.08108):**
Critical findings for VLM attacks:
- Single perturbation can deceive VLMs across both images AND text inputs
- Target value vectors in middle-to-late layers of vision encoder
- Label-free optimization using only input images
- Black-box to LLM while optimizing on vision encoder alone
- Achieves high attack success rates across classification, captioning, VQA

**Robust CLIP (arXiv:2402.12336):**
- Adversarial fine-tuning improves VLM robustness
- Understanding defense mechanisms informs attack design
- Ensemble of CLIP variants provides stronger attack surface

**VLM Adversarial Transferability (arXiv:2403.10883):**
- Collaborative multimodal interaction improves transferability
- Cross-model attacks more effective than single-model
- Attention-based perturbations transfer better

### Proposed Updates to M5

1. **Implement Doubly-UAP for VLM attacks:**
   ```python
   class DoublyUAP:
       def __init__(self, vision_encoder, target_layers="mid_to_late"):
           self.encoder = vision_encoder
           self.target_layers = target_layers
       
       def optimize(self, images, epsilon=16/255, iterations=1000):
           # Target value vectors in attention mechanism
           # Label-free optimization
           # Returns universal perturbation
           pass
       
       def apply(self, image):
           # Apply perturbation to any image
           # Effective across diverse text queries
           pass
   ```

2. **Add multi-modal attack ensemble:**
   - Vision encoder attacks (Doubly-UAP style)
   - Cross-modal attention disruption
   - Embedding space perturbations
   - Combine for maximum transferability

3. **Implement scene-conditioned universal perturbations:**
   ```python
   SCENE_PERTURBATIONS = {
       "urban": "uap_urban.pt",
       "natural": "uap_natural.pt",
       "indoor": "uap_indoor.pt",
       "coastal": "uap_coastal.pt",
       "mountain": "uap_mountain.pt"
   }
   ```

4. **Add VLM-specific loss functions:**
   ```python
   def vlm_attack_loss(perturbed_image, original_image, vlm):
       # Maximize geolocation error
       geo_loss = -haversine_distance(
           vlm.geolocate(perturbed_image),
           vlm.geolocate(original_image)
       )
       # Maintain semantic consistency
       semantic_loss = clip_similarity(perturbed_image, original_image)
       # Disrupt attention patterns
       attention_loss = attention_divergence(perturbed_image, original_image)
       return geo_loss + lambda1 * semantic_loss + lambda2 * attention_loss
   ```

5. **Implement transferability optimization:**
   - Train on ensemble of VLMs (LLaVA, GPT-4V proxy, Gemini proxy)
   - Use collaborative multimodal interaction
   - Validate transfer to held-out models

**New references for M5:**
- [78]: arXiv:2412.08108 "Doubly-Universal Adversarial Perturbations"
- [77]: arXiv:2402.12336 "Robust CLIP"
- [80]: arXiv:2403.10883 "Improving Adversarial Transferability of VLP Models"

---

## M6 — End-to-End Feedback Loop & Policy

### Key Findings from Literature

**Vision-Language Pre-training Survey (arXiv:2210.09263):**
- Understanding VLP architectures informs feedback design
- Attention mechanisms provide interpretability for debugging
- Multi-task learning enables efficient policy optimization

**Multimodal Reasoning Survey (arXiv:2505.04921):**
- Chain-of-thought reasoning in VLMs
- Planning and verification loops
- Self-correction mechanisms

### Proposed Updates to M6

1. **Implement adaptive feedback controller:**
   ```python
   class AdaptiveFeedbackController:
       def __init__(self, target_privacy_tier, max_iterations=10):
           self.target = target_privacy_tier
           self.max_iter = max_iterations
       
       def run(self, image):
           for i in range(self.max_iter):
               # Evaluate current privacy level
               privacy_score = self.evaluate(image)
               
               if privacy_score >= self.target:
                   return image
               
               # Select intervention based on gap
               intervention = self.select_intervention(privacy_score, self.target)
               image = intervention.apply(image)
           
           return image
   ```

2. **Add policy learning from feedback:**
   - Collect success/failure data from iterations
   - Learn optimal intervention sequences
   - Adapt to image characteristics (scene type, geo-cue density)

3. **Implement multi-objective optimization:**
   - Privacy efficacy (primary)
   - Visual quality (constraint)
   - Runtime budget (constraint)
   - Pareto-optimal policy selection

**New references for M6:**
- [84]: arXiv:2210.09263 "Vision-Language Pre-training Survey"
- [87]: arXiv:2505.04921 "Large Multimodal Reasoning Models Survey"

---

## M7 — Robustness & Red-Team Testing

### Key Findings from Literature

**JPEG Compression Effects (arXiv:1803.00940, arXiv:2402.16586):**
- JPEG compression can neutralize adversarial perturbations
- Interpolation smoothing improves JPEG resistance
- Quality factors 75-95 most relevant for social media

**Frequency-Based Defenses (arXiv:2110.13935):**
- Adversarial perturbations often concentrate in specific frequencies
- Frequency filtering can detect/remove perturbations
- Need to design perturbations robust to frequency analysis

**Systematic Defense Evaluation (arXiv:1812.01804):**
- Many defenses fail under adaptive attacks
- Need comprehensive evaluation methodology
- Report both white-box and black-box results

### Proposed Updates to M7

1. **Expand compression robustness testing:**
   ```python
   COMPRESSION_TESTS = {
       "jpeg": [95, 90, 85, 80, 75, 70],
       "webp": [95, 90, 85, 80],
       "heic": [95, 90, 85],
       "resize": [0.9, 0.8, 0.7, 0.5],
       "crop": [0.95, 0.9, 0.85, 0.8, 0.7]
   }
   ```

2. **Add frequency-domain robustness:**
   - Test against frequency filtering defenses
   - Ensure perturbations spread across frequency bands
   - Validate against DCT-based detection

3. **Implement adaptive attack evaluation:**
   - White-box attacks against our own defenses
   - Gradient-based adaptive attacks
   - Query-based black-box attacks

4. **Add social media pipeline simulation:**
   ```python
   SOCIAL_MEDIA_PIPELINES = {
       "twitter": {"resize": 4096, "jpeg_quality": 85},
       "instagram": {"resize": 1080, "jpeg_quality": 80},
       "facebook": {"resize": 2048, "jpeg_quality": 85},
       "whatsapp": {"resize": 1600, "jpeg_quality": 70}
   }
   ```

**New references for M7:**
- [88]: arXiv:1803.00940 "Protecting JPEG Images Against Adversarial Attacks"
- [89]: arXiv:2402.16586 "JPEG-resistance of Adversarial Attacks"
- [90]: arXiv:2110.13935 "Frequency Centric Defense Mechanisms"

---

## M8 — Packaging: CLI + Minimal Desktop UI

### Key Findings from Literature

**Usable Privacy (arXiv:2004.07359, arXiv:2105.02793):**
- Privacy tools often fail due to poor usability
- Users need clear feedback on privacy level achieved
- Defaults should be privacy-preserving
- Transparency about what changes were made

**Privacy Decision Making (arXiv:2003.08990):**
- Users feel disempowered by complex privacy controls
- Simple presets preferred over granular options
- Visual feedback improves understanding

### Proposed Updates to M8

1. **Implement privacy-first defaults:**
   ```python
   DEFAULT_CONFIG = {
       "preset": "balanced",  # Not "light"
       "show_changes": True,
       "confirm_before_save": True,
       "preserve_original": True
   }
   ```

2. **Add visual privacy feedback:**
   - Before/after comparison view
   - Heatmap of modified regions
   - Privacy score indicator (1-10 scale)
   - Estimated geolocation accuracy before/after

3. **Implement progressive disclosure:**
   - Level 1: Simple preset selection (Light/Balanced/Max)
   - Level 2: Category toggles (metadata, text, features, perturbations)
   - Level 3: Advanced parameters (epsilon, iterations, thresholds)

4. **Add transparency features:**
   - Log of all modifications made
   - Explanation of why each change was needed
   - Option to selectively undo changes

**New references for M8:**
- [95]: arXiv:2004.07359 "Usable, Acceptable, Appropriable Privacy"
- [94]: arXiv:2105.02793 "Holistic Privacy and Usability"
- [98]: arXiv:2003.08990 "Privacy-Sharing Perceptions"

---

## M9 — Documentation & Release

### Key Findings from Literature

**Responsible AI Governance (arXiv:2109.05658, arXiv:2004.11434):**
- Measurement-based governance enables accountability
- Stakeholder engagement critical for responsible release
- Clear documentation of capabilities and limitations

**Ethics-Based Auditing (arXiv:2111.04380, arXiv:2110.10980):**
- Define intervention points for ethical review
- Document scope and limitations of auditing
- Establish ongoing monitoring procedures

**AI Safety Reports (arXiv:2412.05282):**
- International coordination on AI safety
- Transparency in capability reporting
- Responsible disclosure practices

### Proposed Updates to M9

1. **Implement model card for Photo Delocalizer:**
   ```markdown
   ## Model Card: Photo Delocalizer v0.1.0
   
   ### Intended Use
   - Privacy protection for individuals sharing photos online
   - NOT for: evading law enforcement, hiding criminal activity
   
   ### Capabilities
   - Defeats CNN-based geolocators with >95% success
   - Defeats VLM-based geolocators with >90% success
   - Maintains LPIPS < 0.06 visual quality
   
   ### Limitations
   - May not defeat future geolocators
   - Requires periodic model updates
   - Some images may have irreducible geo-cues
   
   ### Ethical Considerations
   - Dual-use potential acknowledged
   - Usage logging recommended for enterprise
   ```

2. **Add responsible disclosure policy:**
   - 90-day disclosure window for vulnerabilities
   - Coordinated disclosure with geolocator developers
   - Public security advisories for bypasses

3. **Implement usage guidelines:**
   - Legitimate use cases documented
   - Prohibited uses clearly stated
   - Enterprise deployment requirements

4. **Add audit trail features:**
   - Optional logging of all processing
   - Cryptographic hashing of inputs/outputs
   - Compliance reporting for enterprise

**New references for M9:**
- [101]: arXiv:2109.05658 "Measurement as governance in responsible AI"
- [104]: arXiv:2111.04380 "Ethics-Based Auditing: Intervention Points"
- [106]: arXiv:2412.05282 "International Scientific Report on AI Safety"

---

## Summary of Key Updates

| Milestone | Key Addition | Impact |
|-----------|--------------|--------|
| M0 | VLM threat modeling, granular privacy tiers | Foundation for all subsequent work |
| M1 | Cross-view benchmarks, VLM evaluation | More comprehensive baseline coverage |
| M2 | Selective text removal (SSTR) | Targeted privacy without over-editing |
| M3 | Expanded geo-cue prompts, SAM robustness | Better feature detection |
| M4 | Multi-stage inpainting, anti-geo verification | Higher quality edits |
| M5 | Doubly-UAP for VLMs, scene-conditioned UAPs | State-of-the-art adversarial attacks |
| M6 | Adaptive feedback, policy learning | Smarter intervention selection |
| M7 | Social media pipeline simulation | Real-world robustness |
| M8 | Privacy-first defaults, visual feedback | Better user experience |
| M9 | Model card, responsible disclosure | Ethical release practices |

---

## References

See ADDITIONAL_SCHOLARLY_SOURCES.md for the complete list of 80 additional references organized by milestone.
