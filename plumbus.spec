%global debug_package %{nil}

Name: plumbus
Version: 1.0.5
Release: 1%{?dist}
License: MIT
Summary: Copy to your clipboard using a plumbus

# Clone this repo
# cd plumbus && tito build --tgz
Source0: %{name}-%{version}.tar.gz

BuildRequires: rust cargo python3 libxcb libxcb-devel

%description
Copy to your clipboard using a plumbus

%prep
%autosetup

%build
cargo build --release
strip -s ./target/release/%{name}

%install
rm -rf $RPM_BUILD_ROOT
mkdir -p $RPM_BUILD_ROOT/%{_bindir}
cp ./target/release/%{name} $RPM_BUILD_ROOT/%{_bindir}

%clean
rm -rf $RPM_BUILD_ROOT

%files
%{_bindir}/%{name}

%changelog
* Wed Sep 21 2022 egg <egg95@protonmail.com> 1.0.5-1
- bumped version number (luther.jaymen@gmail.com)
- added verbose option and error prop (luther.jaymen@gmail.com)
- Update README.md (45681670+Radical-Egg@users.noreply.github.com)
* Sun Sep 18 2022 egg <egg95@protonmail.com> 1.0.4-1
- updated main (egg95@protonmail.com)
- added file copy function (egg95@protonmail.com)

* Fri Sep 16 2022 egg <egg95@protonmail.com> 1.0.2-1
- changelog barfed (egg95@protonmail.com)

* Fri Sep 16 2022 egg <egg95@protonmail.com>
- changelog barfed (egg95@protonmail.com)

* Fri Sep 16 2022 egg <egg95@protonmail.com>
- initial build

* Fri Sep 16 2022 egg <egg95@protonmail.com> 1.0.1-1
- updated build deps (egg95@protonmail.com)

* Fri Sep 16 2022 egg <egg95@protonmail.com> 1.0.0-1
- new package built with tito

* Fri Sep 16 2022 egg <egg95@protonmail.com>
- updated readme and builds deps (egg95@protonmail.com)

- new package built with tito

