%global debug_package %{nil}

Name: plumbus
Version: 1.0.0
Release: 1%{?dist}
License: MIT
Summary: Copy to your clipboard using a plumbus

# Clone this repo
# cd plumbus && tito build --tgz
Source0: %{name}-%{version}.tar.gz

BuildRequires: rust cargo

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