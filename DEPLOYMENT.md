# BumbleBees Deployment Guide

This document explains how to deploy BumbleBees to your web server securely.

## Security Notice

**IMPORTANT**: The deployment script and credentials are **NOT** stored in the git repository. This prevents accidental exposure of sensitive information.

## Setup

### 1. Copy the Environment Template

```bash
cp .env.example .env
```

### 2. Edit `.env` with Your Credentials

Open `.env` in your text editor and fill in your actual FTP credentials:

```bash
FTP_HOST="your-server.com"
FTP_USER="your-ftp-username"
FTP_PASS="your-ftp-password"
FTP_PATH="/game"
```

**Note**: Keep quotes around values, especially if they contain special characters.

### 3. Copy the Deployment Script

The deployment script `deploy.sh` should be created locally and is automatically ignored by git:

```bash
# The script is already in your local repository
# Just make sure it's executable:
chmod +x deploy.sh
```

## Usage

### Deploy Everything

To build WASM and upload all files to your server:

```bash
./deploy.sh
```

The script will:
1. ✅ Build WASM in release mode (optimized)
2. ✅ Upload `bumblebees.wasm` (game binary)
3. ✅ Upload `game.html` (game page)
4. ✅ Upload audio files (`intro.ogg`, `music_background.ogg`)
5. ✅ Verify deployment

### What Gets Uploaded

By default, the script uploads:
- `bumblebees.wasm` - WASM game binary (~954KB)
- `game.html` - Game HTML page
- `resources/intro.ogg` - Intro music (1.8MB)
- `resources/music_background.ogg` - Background music (2.5MB)

### Upload Additional Resources

If you need to upload other resources (sprites, sounds, etc.), uncomment the relevant lines in `deploy.sh`:

```bash
# Uncomment these lines to upload additional resources:
# upload_file "resources/sfx_shoot.wav" "${FTP_PATH}/resources/sfx_shoot.wav" "sfx_shoot.wav"
# upload_file "resources/sfx_hit.wav" "${FTP_PATH}/resources/sfx_hit.wav" "sfx_hit.wav"
```

## Manual Deployment

If you prefer to deploy manually without the script:

```bash
# Build WASM
cargo build --target wasm32-unknown-unknown --release

# Upload files manually (use your preferred FTP client)
# - target/wasm32-unknown-unknown/release/bumblebees.wasm → /game/bumblebees.wasm
# - game.html → /game/game.html
# - resources/*.ogg → /game/resources/*.ogg
```

## Troubleshooting

### "Missing required environment variables"

Make sure `.env` file exists and contains all required variables:
- `FTP_HOST`
- `FTP_USER`
- `FTP_PASS`
- `FTP_PATH`

### "WASM build failed"

Ensure you have the WASM target installed:

```bash
rustup target add wasm32-unknown-unknown
```

### FTP Upload Failed

1. Check your credentials in `.env`
2. Verify the FTP path exists on your server
3. Test FTP access manually:

```bash
curl ftp://your-server.com/game/ --user username:password --list-only
```

### Special Characters in Password

If your password contains special characters, make sure it's properly quoted in `.env`:

```bash
# Correct (with quotes)
FTP_PASS="my$p@ss#word"

# Wrong (without quotes)
FTP_PASS=my$p@ss#word
```

## Security Best Practices

1. **Never commit `.env`** - It's in `.gitignore` for a reason
2. **Never commit `deploy.sh`** - Also in `.gitignore`
3. **Change your FTP password** if it's ever exposed
4. **Use strong passwords** with special characters
5. **Consider SSH/SFTP** instead of FTP for better security

## Alternative: SSH/SCP Deployment

For more secure deployment, consider using SSH/SCP instead of FTP:

```bash
# Upload via SCP
scp target/wasm32-unknown-unknown/release/bumblebees.wasm user@server:/path/to/game/
scp game.html user@server:/path/to/game/
scp resources/*.ogg user@server:/path/to/game/resources/
```

## Files That Are NOT in Git

These files are intentionally excluded from version control:

- ✅ `.env` - Your actual credentials (NEVER commit!)
- ✅ `deploy.sh` - Deployment script (NEVER commit!)
- ✅ `.env.example` - Template (safe to commit, no real credentials)

## Questions?

If you encounter issues with deployment, check:
1. `.gitignore` - Ensure deployment files are properly excluded
2. `.env` - Verify all credentials are correct
3. `deploy.sh` - Check the script has executable permissions (`chmod +x deploy.sh`)

---

**Remember**: The deployment script and credentials are for local use only and should never be pushed to GitHub!
