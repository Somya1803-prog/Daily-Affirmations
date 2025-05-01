# Daily Affirmations

## Table of Contents
- [Project Title](#project-title)
- [Project Description](#project-description)
- [Project Vision](#project-vision)
- [Key Features](#key-features)
- [Contract Details](#contract-details)

## Project Title

**Daily Affirmations**

## Project Description

A decentralized Soroban smart contract to help users store and share motivational quotes and positive affirmations on-chain. Perfect for boosting morale and spreading good vibes in a trustless environment.

## Project Vision

To encourage daily positivity and mental wellness through a decentralized system that allows anyone to log, share, and revisit affirmations that inspire them—forever preserved on-chain.

## Key Features

- 💬 **Submit Affirmations**: Authenticated users can log motivational quotes or personal affirmations.
- 📖 **View Entries**: See affirmations you've submitted anytime.
- 🔢 **Count Entries**: Keep track of how many affirmations you’ve added.
- 🧘 **Self-Growth Focused**: Ideal for daily routines or mental health practices.

## Contract Details

### Contract Address: CC745G2WSR4AKEEIKLKQ3JGDRIYXXYSRPMJUPRMOKP3I2NSJOBKUHZ2M
This Soroban contract includes:

### 1. `add_affirmation(user: Address, affirmation: String)`
Stores a new affirmation for the user.

### 2. `get_affirmations(user: Address) -> Vec<String>`
Returns all affirmations submitted by the user.

### 3. `count_affirmations(user: Address) -> u32`
Returns how many affirmations a user has submitted.

---

**Fuel your mind daily. One affirmation at a time.** 🌞✨  
Built with [Soroban](https://soroban.stellar.org) to empower positivity on the blockchain.
