# delocaliser

Privacy-preserving photo post-processing tool designed to defeat visual geolocation systems.

## Overview

delocaliser processes images to prevent AI-based geolocators (VLMs, CNNs, image-retrieval systems) from accurately inferring location, while maintaining human-perceived image quality. The goal is to make predicted locations coarser than 1 degree latitude/longitude (worse than city-level).

## Status

This project is in early development. See the [Roadmap](https://github.com/EditTogether/delocaliser/issues/1) for planned milestones.

## Project Structure

```
delocaliser/
├── core/       # Pipeline orchestration and shared utilities
├── attacks/    # Adversarial perturbation generation
├── rewrite/    # Content rewriting and inpainting
├── eval/       # Evaluation harness and benchmarks
├── ui/         # CLI and desktop UI
└── docs/       # Documentation and threat model
```

## Privacy Tiers

| Tier | Max Accuracy @ 100km | Median Error | Use Case |
|------|---------------------|--------------|----------|
| Country | <5% | >500 km | Travel influencers |
| City | <1% @ 25km | >100 km | General users |
| Neighborhood | <0.1% @ 1km | >25 km | Privacy-sensitive |
| Building | 0% @ 100m | >10 km | High-security |

## Documentation

- [Threat Model](docs/threat_model.md) - Attacker classes, attack surfaces, and success metrics

## Building

```bash
cargo build --release
```

## Testing

```bash
cargo test
```

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please see the issue templates for:
- [Bug Reports](.github/ISSUE_TEMPLATE/bug_report.md)
- [Feature Requests](.github/ISSUE_TEMPLATE/feature_request.md)
- [Evaluation Requests](.github/ISSUE_TEMPLATE/evaluation_request.md)

## References

Key papers informing this project:

- PlaNet (arXiv:1602.05314): CNN-based global geolocation
- PIGEON (CVPR 2024): State-of-art image geolocation
- GeoShield (arXiv:2508.03209): Adversarial perturbations for VLM geo-privacy
- Doubly-UAP (arXiv:2412.08108): Universal perturbations for VLMs

See [docs/threat_model.md](docs/threat_model.md) for complete references.
