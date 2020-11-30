FROM fedora:32

# install base c++ dependencies
RUN dnf install -y \
        gcc.x86_64 \
        ninja-build.x86_64 \
        cmake.x86_64;

RUN dnf clean all;

ARG USERNAME=user
ARG USERID=1000

# setup user and directories
RUN groupadd --gid ${USERID} ${USERNAME} \
    && useradd --uid ${USERID} \
               --gid ${USERNAME} \
               --shell /bin/bash \
               --create-home ${USERNAME};

# switch user and workdirectory
USER ${USERNAME}
WORKDIR /home/${USERNAME}
COPY entrypoint.sh /opt/entrypoint.sh

# install rust with rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENTRYPOINT ["/opt/entrypoint.sh"]
