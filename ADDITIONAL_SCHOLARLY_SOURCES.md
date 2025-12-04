# Additional Scholarly Sources for Photo Delocalizer Roadmap

This document provides additional scholarly sources for each milestone (M0-M9) of the Photo Delocalizer project, organized to complement the existing ~30 references in issue #1.

---

## M0 — Project Kickoff & Threat Model

**Additional sources for privacy threat modeling and geolocation attack taxonomies:**

[31]: https://arxiv.org/abs/2407.04952 "Granular Privacy Control for Geolocation with Vision Language Models"
- Mendes et al. (2024) - Directly addresses privacy control for image geolocation with VLMs, proposing granular privacy mechanisms.

[32]: https://arxiv.org/abs/2408.09474 "Image-Based Geolocation Using Large Vision-Language Models"
- Liu et al. (2024) - Analyzes how LVLMs perform image-based geolocation, providing threat model insights.

[33]: https://arxiv.org/abs/1912.02085 "Protecting Geolocation Privacy of Photo Collections"
- Directly relevant to protecting geolocation privacy in photo collections.

[34]: https://arxiv.org/abs/2010.10139 "Image Obfuscation for Privacy-Preserving Machine Learning"
- Raynal et al. (2020) - Foundational work on image obfuscation techniques for privacy preservation.

[35]: https://arxiv.org/abs/2102.11072 "Obfuscation of Images via Differential Privacy: From Facial Images to General Images"
- Croft et al. (2021) - Differential privacy approaches for image obfuscation.

[36]: https://arxiv.org/abs/2201.09338 "Building a Privacy-Preserving Smart Camera System"
- Beugin et al. (2022) - Privacy-preserving visual systems design.

[37]: https://arxiv.org/abs/2407.13725 "Time-Efficient Locally Relevant Geo-Location Privacy Protection"
- Qiu et al. (2024) - Recent work on efficient geo-location privacy protection.

[38]: https://arxiv.org/abs/1806.09786 "Social Media and User Privacy"
- Beigi (2018) - Survey on social media privacy threats including image-based location inference.

---

## M1 — Baselines & Evaluation Harness

**Additional sources for geolocation benchmarks and evaluation metrics:**

[39]: https://arxiv.org/abs/2104.14995 "Interpretable Semantic Photo Geolocation"
- Provides interpretable approaches to photo geolocation with evaluation frameworks.

[40]: https://arxiv.org/abs/2203.16291 "AmsterTime: A Visual Place Recognition Benchmark Dataset for Severe Domain Shift"
- Benchmark dataset for visual place recognition under domain shift conditions.

[41]: https://arxiv.org/abs/2406.09722 "Cross-view geo-localization: a survey"
- Comprehensive 2024 survey on cross-view geo-localization methods and evaluation.

[42]: https://arxiv.org/abs/2308.16906 "Fine-Grained Cross-View Geo-Localization Using a Correlation-Aware Homography Estimator"
- Fine-grained geo-localization evaluation methodology.

[43]: https://arxiv.org/abs/2102.09186 "Hierarchical Attention Fusion for Geo-Localization"
- Hierarchical approaches to geo-localization with evaluation metrics.

[44]: https://arxiv.org/abs/1703.07815 "Cross-View Image Matching for Geo-localization in Urban Environments"
- Cross-view matching evaluation in urban settings.

[45]: https://arxiv.org/abs/2306.02994 "Long-range UAV Thermal Geo-localization with Satellite Imagery"
- Geo-localization evaluation across different imaging modalities.

[46]: https://arxiv.org/abs/2412.06781 "Around the World in 80 Timesteps: A Generative Approach to Global Visual Geolocation"
- Recent generative approach to global visual geolocation (2024).

---

## M2 — Metadata & Obvious-Cue Sanitization

**Additional sources for EXIF privacy and text detection/removal:**

[47]: https://arxiv.org/abs/2301.04647 "EXIF as Language: Learning Cross-Modal Associations Between Images and Camera Metadata"
- Understanding EXIF metadata as a privacy vector through cross-modal learning.

[48]: https://arxiv.org/abs/1212.3648 "The Metadata Anonymization Toolkit"
- Foundational work on metadata anonymization tools and techniques.

[49]: https://arxiv.org/abs/2309.00410 "Selective Scene Text Removal"
- State-of-the-art selective text removal from scene images.

[50]: https://arxiv.org/abs/2011.09768 "Scene text removal via cascaded text stroke detection and erasing"
- Cascaded approach to scene text removal.

[51]: https://arxiv.org/abs/1710.10400 "Total-Text: A Comprehensive Dataset for Scene Text Detection and Recognition"
- Benchmark dataset for scene text detection evaluation.

[52]: https://arxiv.org/abs/2506.21002 "Inverse Scene Text Removal"
- Recent advances in scene text removal (2025).

[53]: https://arxiv.org/abs/2108.01343 "I3CL: Intra- and Inter-Instance Collaborative Learning for Arbitrary-shaped Scene Text Detection"
- Advanced text detection for arbitrary shapes.

[54]: https://arxiv.org/abs/2504.08616 "Preserving Privacy Without Compromising Accuracy: Machine Unlearning for Handwritten Text Recognition"
- Privacy-preserving approaches to text recognition.

---

## M3 — Geo-Feature Detection & Masking

**Additional sources for visual saliency, geo-cue detection, and open-vocabulary segmentation:**

[55]: https://arxiv.org/abs/2305.08196 "A Comprehensive Survey on Segment Anything Model for Vision and Beyond"
- Comprehensive SAM survey for understanding segmentation capabilities.

[56]: https://arxiv.org/abs/2306.06211 "A Survey on Segment Anything Model (SAM): Vision Foundation Model Meets Prompt Engineering"
- SAM survey focusing on prompt engineering for segmentation.

[57]: https://arxiv.org/abs/2405.10300 "Grounding DINO 1.5: Advance the 'Edge' of Open-Set Object Detection"
- Latest Grounding DINO advances for open-set detection.

[58]: https://arxiv.org/abs/2303.08131 "A Simple Framework for Open-Vocabulary Segmentation and Detection"
- Simple framework for open-vocabulary segmentation.

[59]: https://arxiv.org/abs/2306.15880 "Towards Open Vocabulary Learning: A Survey"
- Survey on open vocabulary learning methods.

[60]: https://arxiv.org/abs/2109.01980 "Deep Saliency Prior for Reducing Visual Distraction"
- Deep saliency methods for identifying visually salient regions.

[61]: https://arxiv.org/abs/1801.04261 "Deep saliency: What is learnt by a deep network about saliency?"
- Understanding what deep networks learn about visual saliency.

[62]: https://arxiv.org/abs/1906.04087 "Large-scale Landmark Retrieval/Recognition under a Noisy and Diverse Dataset"
- Landmark recognition at scale for geo-cue detection.

[63]: https://arxiv.org/abs/2108.08874 "Towards A Fairer Landmark Recognition Dataset"
- Considerations for landmark recognition datasets.

[64]: https://arxiv.org/abs/2305.00866 "Attack-SAM: Towards Attacking Segment Anything Model With Adversarial Examples"
- Understanding SAM vulnerabilities for robust deployment.

---

## M4 — Content Rewriting/Inpainting

**Additional sources for image inpainting quality metrics and semantic consistency:**

[65]: https://arxiv.org/abs/2201.09865 "RePaint: Inpainting using Denoising Diffusion Probabilistic Models"
- Foundational diffusion-based inpainting method.

[66]: https://arxiv.org/abs/2208.03382 "Keys to Better Image Inpainting: Structure and Texture Go Hand in Hand"
- Structure and texture considerations for high-quality inpainting.

[67]: https://arxiv.org/abs/2210.11427 "DiffEdit: Diffusion-based semantic image editing with mask guidance"
- Semantic image editing with diffusion models.

[68]: https://arxiv.org/abs/2111.14818 "Blended Diffusion for Text-driven Editing of Natural Images"
- Text-driven image editing for content rewriting.

[69]: https://arxiv.org/abs/2406.14555 "A Survey of Multimodal-Guided Image Editing with Text-to-Image Diffusion Models"
- Comprehensive 2024 survey on multimodal image editing.

[70]: https://arxiv.org/abs/2303.07909 "Text-to-image Diffusion Models in Generative AI: A Survey"
- Survey on text-to-image diffusion models.

[71]: https://arxiv.org/abs/2307.07678 "Both Spatial and Frequency Cues Contribute to High-Fidelity Image Inpainting"
- Spatial and frequency considerations for inpainting quality.

[72]: https://arxiv.org/abs/2303.16765 "MDP: A Generalized Framework for Text-Guided Image Editing by Manipulating the Diffusion Path"
- Framework for controlled image editing.

[73]: https://arxiv.org/abs/2407.03635 "SSP-IR: Semantic and Structure Priors for Diffusion-based Realistic Image Restoration"
- Semantic consistency in diffusion-based image processing.

---

## M5 — Adversarial Perturbation Engine

**Additional sources for adversarial examples, transferability, and robustness:**

[74]: https://arxiv.org/abs/1712.07107 "Adversarial Examples: Attacks and Defenses for Deep Learning"
- Comprehensive survey on adversarial examples.

[75]: https://arxiv.org/abs/1801.04693 "Towards Imperceptible and Robust Adversarial Example Attacks against Neural Networks"
- Imperceptible adversarial perturbations.

[76]: https://arxiv.org/abs/1411.1792 "How transferable are features in deep neural networks?"
- Foundational work on feature transferability.

[77]: https://arxiv.org/abs/2402.12336 "Robust CLIP: Unsupervised Adversarial Fine-Tuning of Vision Embeddings for Robust Large Vision-Language Models"
- Adversarial robustness for CLIP-based models.

[78]: https://arxiv.org/abs/2412.08108 "Doubly-Universal Adversarial Perturbations: Deceiving Vision-Language Models Across Both Images and Text with a Single Perturbation"
- Universal perturbations for VLMs.

[79]: https://arxiv.org/abs/2507.22398 "On the Reliability of Vision-Language Models Under Adversarial Frequency-Domain Perturbations"
- Frequency-domain adversarial perturbations for VLMs.

[80]: https://arxiv.org/abs/2403.10883 "Improving Adversarial Transferability of Vision-Language Pre-training Models through Collaborative Multimodal Interaction"
- Improving adversarial transferability for VLP models.

[81]: https://arxiv.org/abs/1909.08072 "Adversarial Attacks and Defenses in Images, Graphs and Text: A Review"
- Review of adversarial attacks across modalities.

[82]: https://arxiv.org/abs/1911.09665 "Adversarial Examples Improve Image Recognition"
- Using adversarial examples for improved robustness.

[83]: https://arxiv.org/abs/2103.15670 "On the Adversarial Robustness of Vision Transformers"
- Adversarial robustness of vision transformers.

---

## M6 — End-to-End Feedback Loop & Policy

**Additional sources for iterative image processing and policy-based optimization:**

[84]: https://arxiv.org/abs/2210.09263 "Vision-Language Pre-training: Basics, Recent Advances, and Future Trends"
- VLP foundations for feedback loop design.

[85]: https://arxiv.org/abs/2202.07201 "Holistic Adversarial Robustness of Deep Learning Models"
- Holistic robustness evaluation for iterative systems.

[86]: https://arxiv.org/abs/2004.10250 "Certifying Joint Adversarial Robustness for Model Ensembles"
- Certified robustness for ensemble-based approaches.

[87]: https://arxiv.org/abs/2505.04921 "Perception, Reason, Think, and Plan: A Survey on Large Multimodal Reasoning Models"
- Survey on multimodal reasoning for policy design.

---

## M7 — Robustness & Red-Team Testing

**Additional sources for adversarial robustness evaluation and compression effects:**

[88]: https://arxiv.org/abs/1803.00940 "Protecting JPEG Images Against Adversarial Attacks"
- JPEG-based defense mechanisms.

[89]: https://arxiv.org/abs/2402.16586 "Improving the JPEG-resistance of Adversarial Attacks on Face Recognition by Interpolation Smoothing"
- JPEG-resistant adversarial attacks.

[90]: https://arxiv.org/abs/2110.13935 "Frequency Centric Defense Mechanisms against Adversarial Examples"
- Frequency-based defense evaluation.

[91]: https://arxiv.org/abs/1812.01804 "Random Spiking and Systematic Evaluation of Defenses Against Adversarial Examples"
- Systematic defense evaluation methodology.

[92]: https://arxiv.org/abs/2108.11785 "A Hierarchical Assessment of Adversarial Severity"
- Hierarchical adversarial severity assessment.

[93]: https://arxiv.org/abs/2007.08428 "On Adversarial Robustness: A Neural Architecture Search perspective"
- NAS perspective on adversarial robustness.

---

## M8 — Packaging: CLI + Minimal Desktop UI

**Additional sources for privacy tool UX and user-centered privacy design:**

[94]: https://arxiv.org/abs/2105.02793 "Holistic Privacy and Usability of a Cryptocurrency Wallet"
- Privacy and usability considerations for security tools.

[95]: https://arxiv.org/abs/2004.07359 "Usable, Acceptable, Appropriable: Towards Practicable Privacy"
- Framework for practicable privacy tool design.

[96]: https://arxiv.org/abs/2212.08278 "Seeing through Things: Exploring the Design Space of Privacy-Aware Data-Enabled Objects"
- Design space exploration for privacy-aware systems.

[97]: https://arxiv.org/abs/1602.01937 "YOURPRIVACYPROTECTOR: A recommender system for privacy settings in social networks"
- Privacy settings recommendation for user-friendly interfaces.

[98]: https://arxiv.org/abs/2003.08990 "Dis-Empowerment Online: An Investigation of Privacy-Sharing Perceptions & Method Preferences"
- User perceptions of privacy-sharing methods.

[99]: https://arxiv.org/abs/2204.11343 "Applying Digital Twins in Metaverse: User Interface, Security and Privacy Challenges"
- UI challenges for privacy-sensitive applications.

---

## M9 — Documentation & Release

**Additional sources for responsible AI disclosure and privacy tool governance:**

[100]: https://arxiv.org/abs/2004.11434 "Responsible AI and Its Stakeholders"
- Stakeholder considerations for responsible AI.

[101]: https://arxiv.org/abs/2109.05658 "Measurement as governance in and for responsible AI"
- Governance frameworks for responsible AI.

[102]: https://arxiv.org/abs/2009.07262 "Report prepared by the Montreal AI Ethics Institute (MAIEI) on Publication Norms for Responsible AI"
- Publication norms for responsible AI release.

[103]: https://arxiv.org/abs/2302.10816 "Tailoring Requirements Engineering for Responsible AI"
- Requirements engineering for responsible AI systems.

[104]: https://arxiv.org/abs/2111.04380 "Ethics-Based Auditing of Automated Decision-Making Systems: Intervention Points and Policy Implications"
- Ethics-based auditing frameworks.

[105]: https://arxiv.org/abs/2110.10980 "Ethics-Based Auditing of Automated Decision-Making Systems: Nature, Scope, and Limitations"
- Scope and limitations of ethics-based auditing.

[106]: https://arxiv.org/abs/2412.05282 "International Scientific Report on the Safety of Advanced AI (Interim Report)"
- International AI safety report (2024).

[107]: https://arxiv.org/abs/2506.22183 "A Different Approach to AI Safety: Proceedings from the Columbia Convening on Openness in Artificial Intelligence and AI Safety"
- Alternative approaches to AI safety and openness.

[108]: https://arxiv.org/abs/2003.05155 "Towards CRISP-ML(Q): A Machine Learning Process Model with Quality Assurance Methodology"
- ML process model with quality assurance.

[109]: https://arxiv.org/abs/2010.14374 "Explainable Machine Learning for Public Policy: Use Cases, Gaps, and Research Directions"
- Explainability considerations for public-facing tools.

[110]: https://arxiv.org/abs/2306.17063 "Honesty is the Best Policy: On the Accuracy of Apple Privacy Labels Compared to Apps' Privacy Policies"
- Privacy labeling accuracy and transparency.

---

## Summary

This document adds **80 additional scholarly sources** (references [31]-[110]) organized across all 10 milestones (M0-M9). The sources prioritize:

1. **Recent papers (2022-2025)** where available, especially for rapidly evolving areas like VLM geolocation and diffusion-based editing
2. **Foundational works** where they provide essential theoretical grounding
3. **Directly relevant papers** to image geolocation privacy, adversarial robustness, and privacy-preserving image processing
4. **Survey papers** to provide comprehensive coverage of each technical area

Key highlights include:
- **M0**: Added VLM geolocation privacy papers (Mendes et al. 2024, Liu et al. 2024)
- **M1**: Added cross-view geo-localization survey and benchmarks
- **M2**: Added scene text removal and metadata anonymization papers
- **M3**: Added comprehensive SAM and Grounding DINO surveys
- **M4**: Added diffusion-based inpainting and semantic editing papers
- **M5**: Added VLM adversarial attack papers and transferability studies
- **M7**: Added JPEG compression robustness papers
- **M8**: Added usable privacy and UX design papers
- **M9**: Added responsible AI governance and ethics papers
