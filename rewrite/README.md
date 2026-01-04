# Rewrite Module

This module contains content rewriting and inpainting functionality for geo-cue removal.

## Planned Components

- **EXIF scrubber**: GPS and metadata removal (ExifTool bindings)
- **Text removal**: Scene text detection (EAST/Tesseract) and inpainting
- **Geo-saliency detection**: Grounded-SAM with geo-cue prompts
- **Content inpainting**: Stable Diffusion-based region replacement

## Status

- EXIF scrubbing and text removal planned for M2 (Metadata & Obvious-Cue Sanitization)
- Geo-saliency detection planned for M3 (Geo-Feature Detection & Masking)
- Content inpainting planned for M4 (Content Rewriting/Inpainting)
