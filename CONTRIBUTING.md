# Contributing to Bitcoin App Registry (BAR)

**Thank you for considering contributing to BAR!**  
BAR is a **permissionless, community-driven protocol** built on Bitcoin L1. Every contribution helps strengthen censorship-resistant software distribution for the entire open-source ecosystem.

This repository (`github.com/zuyux/bar`) maintains:
- The official BRC-App protocol specification
- Genesis inscription and examples
- Documentation
- Reference indexers / validators
- Integration examples (BBOX, Zapstore, etc.)

---

## Code of Conduct

We follow the [Contributor Covenant Code of Conduct](https://www.contributor-covenant.org/version/2/1/code_of_conduct/).  
Be respectful, constructive, and aligned with Bitcoin’s ethos of sovereignty and permissionlessness.

---

## Ways to Contribute

You can help in many ways:

### 1. Protocol & Specification
- Suggest improvements to the BRC-App schema
- Propose new operations or optional fields
- Review and refine the validation rules
- Help design the BITVMX migration path (v3)

### 2. Code & Tools
- Build or improve BAR indexers (Rust, TypeScript, Python, etc.)
- Create explorers or frontends
- Add support for recursive inscriptions
- Develop libraries for easy inscription creation
- Integrate BAR with BBOX, Zapstore, or other platforms

### 3. Documentation
- Improve this README or the full spec
- Write guides (e.g. “How to run your own BAR indexer”)
- Translate documentation
- Create example inscription JSONs for popular apps

### 4. Testing & Inscriptions
- Test inscription flows with real wallets
- Submit sample registrations (Bitchat, wallets, Lightning tools, etc.)
- Help verify indexer consensus

### 5. Community & Outreach
- Spread awareness on X, Nostr, Bitcoin forums
- Help new developers register their first app
- Write blog posts or threads about BAR

---

## Development Setup

```bash
git clone https://github.com/zuyux/bar.git
cd bar
```

The repo is intentionally lightweight:
- `/spec/` — Official protocol documentation
- `/examples/` — Sample inscriptions
- `/indexer/` — Reference indexer implementations (coming soon)
- `/docs/` — Full documentation

No complex build system is required for most contributions.

---

## Submitting Changes

1. **Fork** the repository
2. Create a **feature branch** (`git checkout -b feature/my-awesome-change`)
3. Make your changes
4. **Test thoroughly** (especially validation logic)
5. Open a **Pull Request** with a clear description

**Important:**
- All protocol changes must preserve **backward compatibility** with existing inscriptions.
- Changes to the core schema require strong justification and community discussion.
- Keep JSON examples valid and under 800 bytes when possible.

### Issue Guidelines
- Use clear, descriptive titles
- Include reproduction steps for bugs
- Tag issues appropriately (`protocol`, `indexer`, `docs`, `integration`, etc.)

---

## Running a BAR Indexer

We encourage everyone to run their own indexer for maximum decentralization.

See:
- `/indexer/README.md` (will be added after first contributions)
- Current validation rules are defined in the [spec](./spec/BAR-Spec.md)

---

## Registering Your App on BAR

Once the genesis is inscribed (see [Genesis](./GENESIS.md)), anyone can register an app by inscribing a `register` operation to the canonical registry address.

Example inscriptions are in the `/examples/` folder.

---

## Community

- **X**: [@zuyuxxyz](https://x.com/zuyuxxyz) (main maintainer)
- **Discussions**: Use GitHub Discussions in this repo
- **Nostr**: Coming soon (npub will be announced after genesis)

---

## License

This project is licensed under the **MIT License** — see [LICENSE](./LICENSE) for details.

All inscriptions created under the `brc-app` protocol are **permanently public domain** on Bitcoin.

---

**BAR is not owned by any single person or company.**  
It belongs to the Bitcoin community.

The only requirement for contributing is **respect for the no-gatekeeper principle**.

Let’s build the sovereign app registry Bitcoin deserves.

**— The BAR Community**
```
