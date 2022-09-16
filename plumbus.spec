%global debug_package %{nil}

Name: plumbus
Version: 1.0.1
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

%changelog* Fri Sep 16 2022 egg <egg95@protonmail.com>
* Fri Sep 16 2022 egg <egg95@protonmail.com> 1.0.1-1
- updated build deps (egg95@protonmail.com)

* Fri Sep 16 2022 egg <egg95@protonmail.com> 1.0.0-1
- new package built with tito

* Fri Sep 16 2022 egg <egg95@protonmail.com>
- updated readme and builds deps (egg95@protonmail.com)

- new package built with tito

