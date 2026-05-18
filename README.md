# Recovery Planner DApp

## Project Description

Recovery Planner is a Soroban-based decentralized application that helps users plan a balanced day by combining morning recovery metrics with task workload. The smart contract evaluates heart rate data and task attributes to provide a daily status, do/don't recommendations, and an optimized task order.

## Project Vision

Our vision is to make daily productivity healthier by using on-chain logic to prioritize recovery and essential work. The app aims to help users avoid overwork, choose the right focus tasks, and maintain a sustainable routine using a transparent smart contract engine.

## Features

- Morning recovery status calculation using current and baseline resting heart rate.
- Task prioritization based on cognitive load, physical load, and importance.
- Personalized recommendations for "Do" and "Don't" guidance.
- Smart contract logic implemented in Soroban for deterministic plan generation.
- Web frontend for input, interaction, and multilingual display.

## Smart Contract ID

Paste the deployed smart contract ID here after deployment:

`SMART CONTRACT ID: CDRZU2VUHFXTKMQYRRJUBI62EHPHTAHC3GO6ZVEKCM5EGD4SSJEFCNCH`

## Notes for Submission

- Title: Recovery Planner DApp
- Description: A recovery-aware daily planning app with Soroban smart contract evaluation.
- Vision: A healthier, blockchain-powered task planning experience.
- Features: metrics input, task workload analysis, recommendation engine, on-chain plan generation.
- Smart Contract ID: add it after deployment to the Stellar testnet.

## How to Deploy

Build and deploy the contract with the Stellar CLI tools:

```bash
stellar contract build
stellar contract deploy --source-account <your-testnet-account>
```

After deployment, copy the contract ID into the Smart Contract ID section above.
