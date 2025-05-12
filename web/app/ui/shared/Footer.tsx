// app/ui/shared/Footer.tsx

'use client';
import { Link } from 'react-router';
import { company } from '~/config/Config';
import { useTranslation } from 'react-i18next';
import { useState, useEffect } from 'react';

export default function Footer() {
  const { t: tFooter, ready: footerReady } = useTranslation('footer');
  const { t: tCompany, ready: companyReady } = useTranslation('company');
  const [mounted, setMounted] = useState(false);

  // Only update and show translations after client-side hydration is complete
  useEffect(() => {
    setMounted(true);
  }, []);

  // Use a simple placeholder during SSR and initial client render
  // to ensure they match exactly, then switch to translations after hydration
  const copyright = mounted ? tFooter('copyright') : 'copyright';
  const slogan = mounted ? tCompany('slogan') : 'slogan';
  const poweredBy = mounted ? tFooter('poweredBy') : 'Powered By';
  const privacyPolicy = mounted ? tFooter('privacyPolicy') : 'Privacy';
  const termsOfService = mounted ? tFooter('termsOfService') : 'Terms';
  const contactUs = mounted ? tFooter('contactUs') : 'Contact';

  return (
    <footer className="text-invalsia-500 py-3 uppercase">
      <div className="w-full mx-auto px-3 flex flex-row">
        {/* Footer content */}
        <div className="grid grid-cols-3 grid-rows-1 w-full">
          {/* Left */}
          <div className="col-span-1 flex flex-row">
            <div className="flex items-center">
              <Link to="/" className="flex items-center">
                <img src="/logo.svg" alt="Company Logo" className="h-8 mr-3" />
              </Link>
            </div>
            <div className="flex flex-col justify-center">
              <p className="flex justify-start text-xs">{copyright}</p>
              <div className="text-[0.7rem] normal-case">
                <p>{slogan}</p>
              </div>
            </div>
          </div>

          {/* Center */}
          <div className="text-[0.7rem] normal-case flex flex-col items-center justify-center">
            <div className="text-center text-[0.7rem] normal-case">
              <Link to="/service/hosting" className="text-xs hover:underline">
                {poweredBy}
              </Link>
            </div>
          </div>

          {/* Right */}
          <div className="flex flex-col items-end">
            <div className="flex justify-start gap-2">
              <Link to="/privacy-policy" className="text-xs hover:underline">
                {privacyPolicy}
              </Link>
              <Link to="/terms-of-service" className="text-xs hover:underline">
                {termsOfService}
              </Link>
              <Link to="/contact-us" className="text-xs hover:underline">
                {contactUs}
              </Link>
            </div>
            <div className="flex justify-end gap-3 text-[0.7rem] normal-case">
              {company.social.twitter && (
                <a href={company.social.twitter} className="text-xs hover:underline cursor-pointer">
                  Twitter
                </a>
              )}

              {company.social.linkedin && (
                <a
                  href={company.social.linkedin}
                  className="text-xs hover:underline cursor-pointer"
                >
                  LinkedIn
                </a>
              )}

              {company.social.github && (
                <a href={company.social.github} className="text-xs hover:underline cursor-pointer">
                  GitHub
                </a>
              )}
            </div>
          </div>
        </div>
      </div>
    </footer>
  );
}
