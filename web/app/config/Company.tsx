// app/config/Company.tsx

'use client';
const company = {
    name: "Invalsia",
    slogan: "Build, host, and grow your web presence with ease.",
    copyright: `© ${new Date().getFullYear()} Limovia. All rights reserved.`,
    poweredBy: "Powered by Weebuild",
    social: {
        twitter: "https://twitter.com/invalsia",
        linkedin: "https://www.linkedin.com/company/invalsia",
        github: "https://github.com/Invalsia",
    },
    contact: {
        address: {
            street: "Sample Street 123",
            postalCode: "12345",
            city: "Berlin",
            country: "Germany",
        },
        email: {
            support: "support@invalsia.com",
            info: "info@invalsia.com",
            sales: "sales@invalsia.com",
        },
        phone: "+46 123 456789"
    },
    legal: {
        vatId: "SE123456789",
        companyRegistration: "HRB 123456",
    }
};

export default company;