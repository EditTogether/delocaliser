# delocaliser

A privacy-preserving photo post-processing tool designed to defeat visual geolocation systems.

## Overview

delocaliser processes images to prevent AI-based geolocators (vision-language models, CNNs, image-retrieval systems) from accurately inferring location, while maintaining human-perceived image quality. The tool addresses the growing privacy concern of location inference from photos shared on social media and messaging platforms.

## The Problem

Modern AI systems can determine where a photo was taken with alarming accuracy, even without GPS metadata. These systems analyze:

- Architectural styles and building materials
- Vegetation and landscaping patterns
- Street signs, license plates, and business names
- Infrastructure like road markings and utility poles
- Subtle environmental cues humans might overlook

Vision-language models like GPT-4o and Claude can combine visual analysis with chain-of-thought reasoning, making them particularly effective at geolocation. Even when direct output is moderated, extracted hints can be fed into geocoding services.

## Solution Approach

delocaliser uses a multi-stage pipeline to reduce available geographic evidence:

1. **Metadata Scrubbing** - Remove EXIF GPS tags, camera info, and timestamps
2. **Scene Text Removal** - Detect and inpaint text that reveals location (signs, plates, storefronts)
3. **Geo-Saliency Detection** - Identify visual elements that leak geographic information
4. **Content Rewriting** - Replace geo-revealing content with neutral alternatives
5. **Adversarial Perturbation** - Add imperceptible noise that confuses geolocation models
6. **Feedback Control** - Verify protection and escalate if needed

## Privacy Presets

| Preset | Description | Use Case |
|--------|-------------|----------|
| **Light** | Minimal intervention, perturbation only | Quick protection, minimal visual change |
| **Balanced** | Minor inpainting + perturbation | General social media sharing |
| **Max** | Full rewrite + perturbation + effects | Maximum privacy, accepts more visual change |

## Success Criteria

- **Privacy:** Top-1 accuracy within 100 km reduced to ≤5% against geolocation models
- **Quality:** LPIPS ≤0.06 (perceptual similarity to original)
- **Robustness:** Protection survives JPEG compression and social media processing
- **Performance:** 1080p image processed in ≤10 minutes (Fast mode)

## Project Status

This project is in early development. See the [Threat Model](docs/threat_model.md) for detailed security analysis.

### Roadmap

| Milestone | Focus | Status |
|-----------|-------|--------|
| M0 | Project Kickoff & Threat Model | In Progress |
| M1 | Baselines & Evaluation Harness | Planned |
| M2 | Metadata & Text Sanitization | Planned |
| M3 | Geo-Feature Detection & Masking | Planned |
| M4 | Content Rewriting/Inpainting | Planned |
| M5 | Adversarial Perturbation Engine | Planned |
| M6 | End-to-End Feedback Loop | Planned |
| M7 | Robustness & Red-Team Testing | Planned |
| M8 | CLI + Desktop UI | Planned |
| M9 | Documentation & Release | Planned |

## Technology Stack

delocaliser is implemented in Rust for performance and cross-platform deployment. Key dependencies include:

- **ExifTool** - Metadata scrubbing
- **Tesseract + EAST** - Scene text detection and OCR
- **Grounding DINO + SAM** - Open-vocabulary geo-cue detection
- **Stable Diffusion** - Semantic inpainting
- **CLIP variants** - Adversarial optimization ensemble

## Installation

*Coming soon - see M8 milestone*

## Usage

*Coming soon - see M8 milestone*

## Contributing

Contributions are welcome! Please read the threat model first to understand the security context.

Areas where help is needed:
- Evaluation benchmark curation
- Geolocation model implementations for testing
- Cross-platform build testing
- Documentation and examples

## References

This project builds on research in adversarial machine learning and visual privacy:

- GeoShield (arXiv:2508.03209) - Adversarial perturbations for VLM geo-privacy
- PIGEON (CVPR 2024) - State-of-art image geolocation
- PlaNet (arXiv:1602.05314) - CNN-based geolocation
- Expectation over Transformation (ICML 2018) - Robust adversarial examples

See [docs/threat_model.md](docs/threat_model.md) for complete references.

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

delocaliser is developed as part of the EditTogether project, focusing on privacy-preserving media tools.
