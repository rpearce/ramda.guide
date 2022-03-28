FROM nixos/nix

RUN echo "experimental-features = nix-command flakes" >> /etc/nix/nix.conf

WORKDIR /service
COPY . .

# Install cachix if cachix.dhall is included
RUN [ -f cachix.dhall ] && \
    mkdir -p /root/.config/cachix/ && \
    mv cachix.dhall /root/.config/cachix/ && \
    nix-env -iA cachix -f "https://cachix.org/api/v1/install" && \
    nix-env -i jq || \
    true

RUN nix build \
  --extra-substituters "https://cache.ngi0.nixos.org https://cache.nixos.org https://hydra.iohk.io https://nix-community.cachix.org https://ramda-guide.cachix.org" \
  --extra-trusted-public-keys "cache.ngi0.nixos.org-1:KqH5CBLNSyX184S9BKZJo1LxrxJ9ltnY2uAs5c/f1MA= cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY= hydra.iohk.io:f/Ea+s+dFdN+3Y/G+FDgSq+a5NEWhJGzdjvKNGv0/EQ= nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs= ramda-guide.cachix.org-1:/5dVw0awIXHGPFOtNIXdSey3BDFMS7USLM/URwtqq5U="

CMD ["nix", "run", ".#watch"]
