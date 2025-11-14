# Security Policy

## Supported Versions

We release patches for security vulnerabilities for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take security seriously. If you discover a security vulnerability, please follow these steps:

### 1. Do NOT Disclose Publicly

Please do not create a public GitHub issue for security vulnerabilities. This helps prevent exploitation before a fix is available.

### 2. Report Privately

Send a detailed report to the UNC-GDSC team via:

- **Email**: [Your contact email - to be configured]
- **GitHub Security Advisory**: Use the "Security" tab in this repository

### 3. Include in Your Report

- Description of the vulnerability
- Steps to reproduce the issue
- Potential impact
- Suggested fix (if any)
- Your contact information

### 4. What to Expect

- **Acknowledgment**: We will acknowledge receipt within 48 hours
- **Communication**: We will keep you informed of our progress
- **Timeline**: We aim to release a fix within 14 days for critical vulnerabilities
- **Credit**: We will credit you in the security advisory (unless you prefer to remain anonymous)

## Security Best Practices

When deploying this URL shortener in production:

### 1. Environment Variables

- Never commit `.env` files
- Use strong, unique values for sensitive configuration
- Rotate credentials regularly

### 2. Database Security

- Use strong database passwords
- Enable SSL/TLS for database connections in production
- Regularly backup your database
- Keep database software up to date

### 3. Network Security

- Always use HTTPS in production
- Implement rate limiting to prevent abuse
- Use a reverse proxy (Nginx, Caddy) with proper security headers
- Configure CORS appropriately

### 4. Input Validation

- Validate all user input
- Sanitize URLs to prevent XSS attacks
- Implement URL blacklisting for malicious domains
- Set maximum URL length limits

### 5. Authentication & Authorization

Consider implementing:
- API key authentication
- JWT tokens
- Rate limiting per user/IP
- Access control lists

### 6. Monitoring & Logging

- Enable comprehensive logging
- Monitor for suspicious activity
- Set up alerts for unusual patterns
- Regularly review logs

### 7. Dependencies

- Regularly update dependencies
- Use `cargo audit` to check for known vulnerabilities
- Enable Dependabot alerts

### 8. Docker Security

- Use official Rust base images
- Run containers as non-root user (already implemented)
- Scan images for vulnerabilities
- Keep base images updated

## Known Security Considerations

### Current Implementation

This is a basic URL shortener implementation. Before deploying to production, consider:

1. **Rate Limiting**: Not implemented by default - add middleware
2. **Authentication**: No authentication - implement as needed
3. **URL Validation**: Basic validation - enhance for production
4. **Malicious URL Detection**: Not implemented - integrate with URL scanning services
5. **DoS Protection**: Limited - implement additional protections

## Security Updates

We will publish security updates in:

- GitHub Security Advisories
- CHANGELOG.md
- Release notes

## Responsible Disclosure

We kindly ask security researchers to:

- Give us reasonable time to address the issue before public disclosure
- Make a good faith effort to avoid privacy violations and data destruction
- Not exploit the vulnerability beyond what is necessary to demonstrate it

## Compliance

This project follows security best practices for:

- OWASP Top 10
- Rust security guidelines
- Container security standards

## Questions?

If you have questions about security but don't have a vulnerability to report, please:

- Open a discussion in the GitHub Discussions tab
- Tag it with the `security` label

Thank you for helping keep this project and its users safe!
