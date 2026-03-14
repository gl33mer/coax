"""
Email Service Module
Handles email sending via SendGrid and Mailgun.

WARNING: This file contains intentional secrets for testing.
"""

import os
import logging
from typing import Dict, List, Optional

logger = logging.getLogger(__name__)

# Email configuration
DEFAULT_SENDER = "noreply@example.com"
EMAIL_TIMEOUT = 30

# SendGrid Configuration (CRITICAL - should be detected)
SENDGRID_API_KEY = "SG.abcdefghijklmnopqrstuv.1234567890abcdefghijklmnopqrstuvwxyzABCDEF"
SENDGRID_SENDER = "noreply@example.com"

# Mailgun Configuration (CRITICAL - should be detected)
MAILGUN_API_KEY = "key-1234567890abcdefghijklmnopqrstuvwxyz1234"
MAILGUN_DOMAIN = "mail.example.com"

# Twilio Configuration (CRITICAL - should be detected)
TWILIO_ACCOUNT_SID = "AC1234567890abcdefghijklmnopqrstuvwxyz12345678"
TWILIO_AUTH_TOKEN = "1234567890abcdef1234567890abcdef"


class EmailService:
    """Email sending service."""

    def __init__(self):
        self.sendgrid_key = SENDGRID_API_KEY
        self.mailgun_key = MAILGUN_API_KEY
        self.default_sender = DEFAULT_SENDER

    def send_email(
        self,
        to: str,
        subject: str,
        body: str,
        html: Optional[str] = None
    ) -> Dict:
        """Send an email."""
        logger.info(f"Sending email to {to}")
        # Email sending logic would go here
        return {
            "success": True,
            "message_id": f"msg_{id(self)}",
            "to": to,
            "subject": subject
        }

    def send_bulk_email(
        self,
        recipients: List[str],
        subject: str,
        body: str
    ) -> Dict:
        """Send emails to multiple recipients."""
        results = []
        for recipient in recipients:
            result = self.send_email(recipient, subject, body)
            results.append(result)
        return {
            "success": True,
            "sent": len(results),
            "results": results
        }


class SMSService:
    """SMS sending service via Twilio."""

    def __init__(self):
        self.account_sid = TWILIO_ACCOUNT_SID
        self.auth_token = TWILIO_AUTH_TOKEN

    def send_sms(self, to: str, body: str) -> Dict:
        """Send an SMS."""
        logger.info(f"Sending SMS to {to}")
        return {
            "success": True,
            "message_id": f"sms_{id(self)}",
            "to": to
        }


def get_email_service() -> EmailService:
    """Get an email service instance."""
    return EmailService()


def get_sms_service() -> SMSService:
    """Get an SMS service instance."""
    return SMSService()
