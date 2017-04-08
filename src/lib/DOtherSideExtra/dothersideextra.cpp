/*
 * Copyright (C) 2016 Lucien XU <sfietkonstantin@free.fr>
 *
 * You may use this file under the terms of the BSD license as follows:
 *
 * "Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *   * Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in
 *     the documentation and/or other materials provided with the
 *     distribution.
 *   * The names of its contributors may not be used to endorse or promote
 *     products derived from this software without specific prior written
 *     permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE."
 */

#include "dothersideextra.h"
#include <algorithm>
#include <cstring>
#include <memory>
#include <vector>

#ifdef SAILFISH
#include <sailfishapp.h>
#else
#include <QGuiApplication>
#include <QQmlEngine>
#include <QQuickView>
#endif

static QGuiApplication * createGuiApplication(int &argc, char **argv) {
#ifdef SAILFISH
    return SailfishApp::application(argc, argv);
#else
    return new QGuiApplication {argc, argv};
#endif
}

static QQuickView * createQuickView() {
#ifdef SAILFISH
    return SailfishApp::createView();
#else
    QQuickView *view {new QQuickView {}};
    QObject::connect(view->engine(), &QQmlEngine::quit,
                     QCoreApplication::instance(), &QCoreApplication::quit);
    return view;
#endif
}

template <typename T>
struct WrappedArray {
    WrappedArray(const T *first, std::ptrdiff_t size)
        : m_begin {first}
        , m_end {first + size}
    {
    }
    const T * begin() const noexcept
    {
        return m_begin;
    }
    const T * end() const noexcept
    {
        return m_end;
    }

    const T *m_begin;
    const T *m_end;
};

class DEApplicationImpl
{
public:
    explicit DEApplicationImpl(int argc, const char * const *argv)
        : m_argc {argc}
    {
        const WrappedArray<const char *> array {argv, static_cast<std::ptrdiff_t>(argc)};
        std::transform(array.begin(), array.end(), std::back_inserter(m_arguments),
                       [](const char *string) {
           return QByteArray {string};
        });
        std::transform(m_arguments.begin(), m_arguments.end(), std::back_inserter(m_argv),
                       [](QByteArray &stringPtr) {
            return stringPtr.data();
        });

        m_application.reset(createGuiApplication(m_argc, m_argv.data()));
    }
    QGuiApplication & application() const
    {
        return *m_application;
    }
private:
    std::unique_ptr<QGuiApplication> m_application {};
    std::vector<QByteArray> m_arguments {};
    int m_argc {0};
    std::vector<char *> m_argv {};
};

DEApplication * de_qguiapplication_create(int argc, const char * const *argv)
{
    return new DEApplicationImpl {argc, argv};
}

void de_qguiapplication_delete(DEApplication *vptr)
{
    delete static_cast<DEApplicationImpl *>(vptr);
}

DosQQuickView * de_qquickview_create()
{
    return createQuickView();
}
