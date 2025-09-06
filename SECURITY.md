# Security Policy

## Overview

Claynosaurz is committed to maintaining the highest security standards for our Solana staking smart contract. This document outlines our security practices, audit results, and vulnerability disclosure process.

## Security Audits

### OtterSec Security Assessment ✅

- **Audit Firm**: [OtterSec](https://osec.io/) - Leading Solana security auditor
- **Audit Period**: January 28 - February 18, 2025
- **Report Date**: February 24, 2025
- **Audited Version**: Commit `b47a8df`
- **Resolution Version**: Commit `0b87d6e`
- **Report**: [View Full Audit Report](audit/claynosaurz_solana_staking_audit_final.pdf)

**Audit Results:**
- ✅ **0 Critical** vulnerabilities
- ✅ **1 High** vulnerability - RESOLVED
- ✅ **3 Medium** vulnerabilities - RESOLVED  
- ✅ **3 Low** vulnerabilities - RESOLVED
- ℹ️ **3 Informational** suggestions implemented

**All identified vulnerabilities have been resolved and verified by OtterSec.**

#### Key Security Improvements Made:
- Enhanced NFT collection integrity validation
- Proper level progression caps enforcement
- Corrected multiplier update mechanisms
- Fixed admin account mutability issues
- Improved account size calculations
- Resolved potential underflow scenarios

## Verified Builds

Our smart contract is deployed with verified builds to ensure transparency:

- **Program ID**: `CLAYVLFC58dsDXcYTy4vD6uK4Gu6xy6UNhUHkXsBkRfu`
- **Verification Status**: ✅ Verified on Solana Explorer
- **Source Code**: Matches deployed bytecode exactly
- **Build Reproducibility**: Deterministic builds using Docker

## Scope

This security policy covers the following components:

### In Scope
- **Claynosaurz Staking Smart Contract** (`CLAYVLFC58dsDXcYTy4vD6uK4Gu6xy6UNhUHkXsBkRfu`)
- NFT staking and unstaking mechanisms
- Experience point calculations and level progression
- Multiplier systems (permanent and ephemeral)
- Admin functions and governance controls
- Token delegation and authority management

### Key Areas of Focus
- **Token Security**: NFT custody and delegation mechanisms
- **Economic Logic**: Point calculations, multipliers, and reward distribution
- **Access Controls**: Admin privileges and user permissions
- **State Management**: Account data integrity and updates
- **Integration Points**: Metaplex Token Metadata interactions

## Reporting Security Vulnerabilities

We take security seriously and appreciate responsible disclosure of potential vulnerabilities.

### How to Report

**Primary Contact:**
- **Email**: security@claynosaurz.com
- **GitHub Security Advisory**: [Create Advisory](https://github.com/Claynosaurz-Inc/staking-smart-contract/security/advisories/new)

**Alternative Contacts:**
- **Discord**: Contact our team in the [Claynosaurz Discord](https://discord.gg/claynosaurz)
- **Twitter**: [@Claynosaurz](https://twitter.com/claynosaurz)

### What to Include in Your Report

Please provide as much detail as possible:

1. **Vulnerability Description**: Clear explanation of the issue
2. **Impact Assessment**: Potential consequences and affected users
3. **Reproduction Steps**: Detailed steps to reproduce the vulnerability
4. **Proof of Concept**: Code, transaction hashes, or screenshots if applicable
5. **Suggested Fix**: If you have ideas for remediation
6. **Your Contact Information**: For follow-up communication

### Response Timeline

We are committed to responding promptly to security reports:

- **Acknowledgment**: Within 48 hours
- **Initial Assessment**: Within 7 days
- **Regular Updates**: Every 7 days during investigation
- **Resolution Timeline**: Varies by severity (see below)

#### Severity-Based Response Times

| Severity | Response Time | Resolution Target |
|----------|---------------|-------------------|
| **Critical** | 24 hours | 7 days |
| **High** | 48 hours | 14 days |
| **Medium** | 7 days | 30 days |
| **Low** | 14 days | 60 days |

## Disclosure Policy

We follow a **90-day coordinated disclosure timeline**:

1. **Day 0**: Vulnerability reported and acknowledged
2. **Day 1-30**: Investigation, validation, and fix development
3. **Day 31-60**: Testing, deployment, and verification of fixes
4. **Day 61-90**: Coordination with reporter on public disclosure
5. **Day 90+**: Public disclosure (unless extension agreed upon)

### Early Disclosure

We may disclose vulnerabilities earlier than 90 days if:
- The fix is deployed and verified
- The vulnerability is being actively exploited
- The reporter agrees to early disclosure

## Bug Bounty Program

We are currently evaluating the implementation of a formal bug bounty program. In the meantime:

- **Recognition**: Security researchers will be acknowledged in our documentation
- **Coordination**: We work closely with reporters throughout the process
- **Future Rewards**: Researchers who report valid vulnerabilities may be eligible for rewards once our program launches

## Out of Scope

The following are **NOT** in scope for security research:

### Technical Exclusions
- **Social Engineering**: Attacks targeting team members or users
- **Physical Security**: Attacks on infrastructure or personnel
- **Denial of Service**: Network flooding or resource exhaustion attacks
- **Spam/Phishing**: Email or social media attacks
- **Third-Party Services**: Issues with external dependencies (RPC providers, etc.)

### Known Limitations
- **Solana Network Issues**: Problems with the underlying Solana blockchain
- **Wallet Vulnerabilities**: Issues with user wallet software
- **Frontend Applications**: Security issues in web interfaces (separate scope)

## Legal Safe Harbor

We provide legal safe harbor for security researchers who:

1. **Act in Good Faith**: Report vulnerabilities responsibly
2. **Avoid Harm**: Do not access, modify, or delete user data
3. **Respect Privacy**: Do not access other users' accounts or data
4. **Follow Disclosure**: Adhere to our coordinated disclosure timeline
5. **Comply with Laws**: Follow applicable local and international laws

### Protection Commitment

We commit to:
- **No Legal Action**: Against researchers following these guidelines
- **Cooperation**: Work with researchers throughout the process
- **Recognition**: Publicly acknowledge contributions (with permission)
- **Communication**: Maintain open and professional dialogue

## Security Best Practices

### For Users
- **Verify Contracts**: Always verify you're interacting with the correct program ID
- **Use Trusted Interfaces**: Only use official Claynosaurz interfaces
- **Check Transactions**: Review all transaction details before signing
- **Keep Software Updated**: Use the latest wallet software and browser versions

### For Developers
- **Code Reviews**: All changes undergo thorough peer review
- **Testing**: Comprehensive test coverage including edge cases
- **Monitoring**: Continuous monitoring of contract interactions
- **Updates**: Regular dependency updates and security patches

## Contact Information

### Security Team
- **Primary**: security@claynosaurz.com
- **Response Time**: 24-48 hours

### Development Team
- **GitHub**: [Claynosaurz-Inc](https://github.com/Claynosaurz-Inc)
- **Issues**: [Report Issues](https://github.com/Claynosaurz-Inc/staking-smart-contract/issues)

### Community
- **Discord**: [Join our community](https://discord.gg/claynosaurz)
- **Twitter**: [@Claynosaurz](https://twitter.com/claynosaurz)
- **Website**: [claynosaurz.com](https://claynosaurz.com)

## Acknowledgments

We thank the following security researchers and organizations:

- **OtterSec Team**: For conducting our comprehensive security audit
  - Xiang Yinsoreatu (yinsoreatu@osec.io)
  - Gabriel Ottoboni (ottoboni@osec.io)
  - Kevin Chow (kchow@osec.io)
  - Robert Chen (r@osec.io)

*Security researchers who responsibly disclose vulnerabilities will be acknowledged here with their permission.*

## Updates

This security policy is regularly reviewed and updated. Last updated: February 2025.

For the most current version, please check: [https://github.com/Claynosaurz-Inc/staking-smart-contract/blob/main/SECURITY.md](https://github.com/Claynosaurz-Inc/staking-smart-contract/blob/main/SECURITY.md)

---

**Remember**: Security is a shared responsibility. If you see something, say something. Together, we can keep the Claynosaurz ecosystem safe and secure.
